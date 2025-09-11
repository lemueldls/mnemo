// import { isTauri } from "@tauri-apps/api/core";
// import TauriWebSocket from "@tauri-apps/plugin-websocket";
import { defu } from "defu";

type WS = WebSocket; // | TauriWebSocket;

export interface WebSocketMessage {
  data: string | ArrayBuffer | Uint8Array;
  bytes(): Promise<Uint8Array>;
  text(): Promise<string>;
}

export interface WebSocketOptions {
  immediate?: boolean;
  protocols?: string[];
  onOpen?: (ws: WS, event: Event) => void;
  onMessage?: (ws: WS, event: WebSocketMessage) => void;
  onError?: (ws: WS, event: Event) => void;
  onClose?: (ws: WS, event: CloseEvent) => void;
  heartbeat?: {
    message?: string | ArrayBuffer | Uint8Array;
    interval?: number;
    pongTimeout?: number;
  };
  autoReconnect?: {
    retries?: number;
    delay?: number;
    onFailed?: () => void;
  };
}

export interface WebSocketReturn {
  // data: Readonly<Ref<string | null>>;
  status: Readonly<Ref<"CONNECTING" | "OPEN" | "CLOSING" | "CLOSED">>;
  close: (code?: number, reason?: string) => Promise<void>;
  open: () => void;
  send: (data: string | Uint8Array) => Promise<boolean>;
  // ws: Readonly<Ref<WS | null>>;
}

class WebSocketMessageWrapper implements WebSocketMessage {
  constructor(public data: string | Uint8Array) {}

  async bytes(): Promise<Uint8Array> {
    if (this.data instanceof Uint8Array) {
      return this.data;
    }

    return new TextEncoder().encode(this.data as string);
  }

  async text(): Promise<string> {
    if (typeof this.data === "string") {
      return this.data;
    }

    const bytes = await this.bytes();

    return new TextDecoder().decode(bytes);
  }
}

export function useApiWebSocket(
  url: MaybeRefOrGetter<string | URL>,
  options: WebSocketOptions = {},
): WebSocketReturn {
  const {
    immediate,
    protocols,
    onOpen,
    onMessage,
    onError,
    onClose,
    heartbeat,
    autoReconnect,
  } = defu(options, {
    immediate: true,
    protocols: [],
    autoReconnect: {
      retries: 5,
      delay: 1000,
      onFailed() {},
    },
  });

  const urlRef = toRef(url);
  // const data = ref<string | ArrayBuffer | null>(null);
  const status = ref<"CONNECTING" | "OPEN" | "CLOSING" | "CLOSED">("CLOSED");
  const ws = ref<WS | null>(null);

  let heartbeatTimer: NodeJS.Timeout | null = null;
  let reconnectTimer: NodeJS.Timeout | null = null;
  let reconnectAttempts = 0;
  let explicitlyClosed = false;

  // const tauri = isTauri();

  const cleanup = () => {
    if (heartbeatTimer) {
      clearTimeout(heartbeatTimer);
      heartbeatTimer = null;
    }
    if (reconnectTimer) {
      clearTimeout(reconnectTimer);
      reconnectTimer = null;
    }
  };

  const startHeartbeat = () => {
    if (!heartbeat) return;

    cleanup();
    heartbeatTimer = setTimeout(() => {
      if (ws.value && status.value === "OPEN") {
        send(heartbeat.message || "ping");
        startHeartbeat();
      }
    }, heartbeat.interval || 30000);
  };

  const attemptReconnect = () => {
    if (explicitlyClosed || !autoReconnect) return;

    if (reconnectAttempts < (autoReconnect.retries || 5)) {
      reconnectAttempts++;
      reconnectTimer = setTimeout(() => {
        if (!explicitlyClosed) {
          open();
        }
      }, autoReconnect.delay || 1000);
    } else {
      autoReconnect.onFailed?.();
    }
  };

  const open = async () => {
    if (ws.value && status.value === "OPEN") return;

    cleanup();
    explicitlyClosed = false;

    const resolvedUrl = unref(urlRef);
    const wsUrl =
      resolvedUrl instanceof URL ? resolvedUrl.toString() : resolvedUrl;

    try {
      // if (tauri) {
      //   ws.value = await TauriWebSocket.connect(wsUrl);

      //   status.value = "OPEN";
      //   reconnectAttempts = 0;

      //   onOpen?.(ws.value as WS, new Event("open"));
      //   startHeartbeat();

      //   // Set up message listener
      //   ws.value.addListener((message) => {
      //     switch (message.type) {
      //       case "Binary": {
      //         const binaryData = new Uint8Array(message.data);
      //         const wrappedMessage = new WebSocketMessageWrapper(binaryData);

      //         // data.value = binaryData;

      //         onMessage?.(ws.value as WS, wrappedMessage);

      //         break;
      //       }

      //       case "Text": {
      //         const textData = message.data;
      //         const wrappedTextMessage = new WebSocketMessageWrapper(textData);

      //         // data.value = textData;

      //         onMessage?.(ws.value as WS, wrappedTextMessage);

      //         break;
      //       }

      //       case "Close": {
      //         status.value = "CLOSED";
      //         cleanup();
      //         onClose?.(ws.value as WS, new CloseEvent("close"));

      //         attemptReconnect();

      //         break;
      //       }
      //     }
      //   });
      // } else {
      // Use native WebSocket
      status.value = "CONNECTING";
      ws.value = new WebSocket(
        wsUrl,
        protocols.length > 0 ? protocols : undefined,
      );

      ws.value.addEventListener("open", (event: Event) => {
        status.value = "OPEN";
        reconnectAttempts = 0;
        onOpen?.(ws.value as WS, event);
        startHeartbeat();
      });

      ws.value.addEventListener(
        "message",
        async (event: MessageEvent<string | Blob>) => {
          const wrappedMessage = new WebSocketMessageWrapper(
            typeof event.data === "string"
              ? event.data
              : await event.data.bytes(),
          );

          // data.value = event.data;

          onMessage?.(ws.value as WS, wrappedMessage);
        },
      );

      ws.value.addEventListener("error", (event: Event) => {
        onError?.(ws.value as WS, event);
      });

      ws.value.addEventListener("close", (event: CloseEvent) => {
        status.value = "CLOSED";
        cleanup();
        onClose?.(ws.value as WS, event);

        attemptReconnect();
      });
      // }
    } catch (error) {
      status.value = "CLOSED";
      console.error("WebSocket connection failed:", error);

      attemptReconnect();
    }
  };

  const close = async (code?: number, reason?: string) => {
    explicitlyClosed = true;
    cleanup();

    if (ws.value) {
      status.value = "CLOSING";
      // if (tauri) {
      //   await (ws.value as TauriWebSocket).disconnect();
      // } else {
      (ws.value as WebSocket).close(code, reason);
      // }
    }

    status.value = "CLOSED";
  };

  const send = async (
    messageData: string | ArrayBuffer | Uint8Array,
  ): Promise<boolean> => {
    await until(ws).toBeTruthy();
    await until(status).toBe("OPEN");

    try {
      // if (tauri) {
      //   let sendData: string | number[];

      //   if (typeof messageData === "string") {
      //     sendData = messageData;
      //   } else if (messageData instanceof ArrayBuffer) {
      //     sendData = Array.from(new Uint8Array(messageData));
      //   } else {
      //     sendData = Array.from(messageData);
      //   }

      //   await (ws.value as TauriWebSocket).send(sendData);
      // } else {
      (ws.value as WebSocket).send(messageData);
      // }

      return true;
    } catch (error) {
      console.error("Failed to send WebSocket message:", error);

      attemptReconnect();

      return false;
    }
  };

  if (immediate) {
    open();
  }

  // Cleanup on unmount
  tryOnScopeDispose(async () => {
    await close();
  });

  return {
    // data: readonly(data),
    status: readonly(status),
    close,
    open,
    send,
    // ws: readonly(ws),
  };
}
