import indexedDbDriver from "unstorage/drivers/indexedb";
import type { Container } from "loro-crdt";
import { createStorage, normalizeKey, type StorageValue } from "unstorage";

const localDb = createStorage({
  driver: indexedDbDriver({ base: "app:" }),
});

const itemRefs: { [key: string]: Ref<unknown> | undefined } = {};

const syncQueue = new Set<string>();

declare global {
  interface Uint8Array {
    toBase64(): string;
  }

  interface Uint8ArrayConstructor {
    fromBase64(this: this, base64: string): Uint8Array;
  }
}

if (!Uint8Array.prototype.toBase64) {
  Uint8Array.prototype.toBase64 = function () {
    let binaryString = "";
    for (let i = 0; i < this.length; i++) {
      binaryString += String.fromCharCode(this[i]!);
    }

    return btoa(binaryString);
  };
}

if (!Uint8Array.fromBase64) {
  Uint8Array.fromBase64 = function (base64String) {
    // Decode the Base64 string to a string of binary data
    const binaryString = atob(base64String);
    // Create a Uint8Array from the binary string
    const len = binaryString.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes;
  };
}

export const useCrdt = createSharedComposable(async () => {
  const { LoroDoc } = await import("loro-crdt");

  const doc = new LoroDoc();

  // const undoManager = new UndoManager(doc, {
  //   maxUndoSteps: 100,
  //   mergeInterval: 1000,
  // });

  const bytes = await localDb.getItem<string>("crdt");
  if (bytes) {
    try {
      doc.import(Uint8Array.fromBase64(bytes));
    } catch {
      await localDb.removeItem("crdt");
    }
  }

  const url = new URL("/api/crdt", useApiBaseUrl());
  url.protocol = url.protocol === "https:" ? "wss:" : "ws:";

  const { open, close, send } = useApiWebSocket(url, {
    immediate: false,
    async onMessage(_ws, event) {
      const bytes = await event.bytes();
      doc.import(bytes);

      await localDb.setItem("crdt", bytes.toBase64());
    },
  });

  const { loggedIn } = useAuth();
  watchImmediate([loggedIn, useOnline()], ([loggedIn, isOnline]) => {
    if (loggedIn && isOnline) open();
    else close();
  });

  doc.subscribeLocalUpdates(async (bytes) => {
    await send(bytes);
  });

  doc.subscribe(async (event) => {
    if (event.by === "import") {
      for (const { path, diff } of event.events) {
        console.log({ path, diff });

        const [key] = path as [string];

        switch (diff.type) {
          case "map": {
            const localItem =
              await localDb.getItem<Record<string, unknown>>(key);
            const item = localItem || {};

            for (const [key, value] of Object.entries(diff.updated))
              if (value === null) delete item[key];
              else item[key] = value;

            await localDb.setItem(key, item);
            await localDb.setMeta(key, { updatedAt: Date.now() });

            const itemRef = itemRefs[key];
            if (itemRef && itemRef.value !== item) {
              syncQueue.add(key);
              itemRef.value = item;
            }

            break;
          }
        }
      }
    }
  });

  return doc;
});

const useSync = createSharedComposable(() => {
  const url = new URL("/api/user-storage", useApiBaseUrl());
  url.protocol = url.protocol === "https:" ? "wss:" : "ws:";

  const { open, close, send } = useApiWebSocket(url, {
    immediate: false,
    async onMessage(_ws, event) {
      const text = await event.text();
      const { key, value, updatedAt } = JSON.parse(text) as {
        key: string;
        value: StorageValue;
        updatedAt: number;
      };

      const meta = await localDb.getMeta(key);

      if (!meta.updatedAt || updatedAt > (meta.updatedAt as number)) {
        await localDb.setItem(key, value);
        await localDb.setMeta(key, { updatedAt });

        const itemRef = itemRefs[key];
        if (itemRef && itemRef.value !== value) {
          syncQueue.add(key);
          itemRef.value = value;
        }
      }
    },
  });

  const { loggedIn } = useAuth();
  watchImmediate([loggedIn, useOnline()], ([loggedIn, isOnline]) => {
    if (loggedIn && isOnline) open();
    else close();
  });

  return {
    async updateItem(key: string, value: StorageValue, updatedAt: number) {
      await send(JSON.stringify({ key, value, updatedAt }));
    },
  };
});

export async function useStorageItem<T extends StorageValue>(
  key: MaybeRefOrGetter<string>,
  initialValue: T,
) {
  const keyRef = toRef(key);

  const { error, data, pending } = await useAsyncData(key, async () => {
    const key = keyRef.value;

    const storageItem = await getStorageItem<T>(key, initialValue);
    const item = ref(storageItem);

    itemRefs[key] = item;

    watchThrottled(
      item,
      async (value) => {
        if (!syncQueue.delete(key)) {
          const updatedAt = Date.now();

          await localDb.setItem(key, value);
          await localDb.setMeta(key, { updatedAt });

          await useSync().updateItem(key, value, updatedAt);
        }
      },
      { throttle: 1000, deep: true },
    );

    return item as Ref<T>;
  });

  watchImmediate(error, (error) => {
    if (error) throw createError(error);
  });

  const root = computed<T>({
    get: (oldValue) => (pending.value ? oldValue! : data.value!.value),
    set: (value) => {
      data.value!.value = value;
    },
  });

  Object.defineProperty(root, "__v_isShallow", {
    configurable: true,
    enumerable: false,
    value: true,
  });

  whenever(logicNot(pending), () => {
    triggerRef(root);
  });

  return root;
}

export async function useStorageText(key: string, initialValue?: string) {
  const item = await useStorageItem(key, initialValue || "");

  const doc = await useCrdt();
  const text = doc.getText(normalizeKey(key));

  const computedRef = computed({
    get: () => item.value,
    set(newValue) {
      item.value = newValue;
      text.update(newValue);

      commit();
    },
  });

  return extendRef(computedRef, {});
}

export async function useStorageMap<T extends object>(
  key: string,
  initialValue?: T,
) {
  const item = await useStorageItem<T>(key, initialValue || ({} as T));

  const doc = await useCrdt();
  const map = doc.getMap(normalizeKey(key));

  const computedRef = computed({
    get: () => item.value,
    set(newValue) {
      item.value = newValue;
      for (const [key, value] of Object.entries(newValue)) {
        map.set(key, value);
      }

      commit();
    },
  });

  return extendRef(computedRef, {
    set(key: string, value: Exclude<T[keyof T], Container>) {
      item.value[key as keyof T] = value;
      map.set(key, value);

      commit();
    },
    delete(key: string) {
      // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
      delete item.value[key as keyof T];
      map.delete(key);

      commit();
    },
  });
}

const commit = useThrottleFn(
  async () => {
    const doc = await useCrdt();
    const bytes = doc.export({ mode: "snapshot" });
    await localDb.setItem("crdt", bytes.toBase64());
  },
  1000,
  true,
  true,
);

export async function getStorageItem<T extends StorageValue>(
  key: string,
  initialValue: T,
) {
  const localItem = await localDb.getItem<T>(key);

  if (!localItem) {
    await localDb.setItem(key, initialValue);
    await localDb.setMeta(key, { updatedAt: 0 });
  }

  const value = localItem || initialValue;

  const localMeta = localDb.getMeta(key) as Promise<
    { updatedAt?: number } | undefined
  >;

  localMeta.then(async (meta) => {
    console.log({ update: meta?.updatedAt });
    await useSync().updateItem(key, value, meta?.updatedAt || 0);
  });

  return value;
}

// const updateRemoteItem = useThrottleFn(async (key, value) => {
//   const { $sync } = useNuxtApp();
//   await $sync.setItem(key, value!);
// }, 500);

// export async function useStorageKeys(base: string) {
//   const { $sync } = useNuxtApp();

//   return await $sync.getKeys(base);
// }
