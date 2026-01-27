import { LoroDoc } from "loro-crdt/bundler";
import * as imports from "loro-crdt/bundler";
import wasm from "loro-crdt/bundler/loro_wasm_bg.wasm?module";

const instance = new WebAssembly.Instance(wasm, {
  "./loro_wasm_bg.js": imports,
});

(
  imports as typeof imports & {
    __wbg_set_wasm(wasm: WebAssembly.Exports): void;
  }
).__wbg_set_wasm(instance.exports);

export default defineWebSocketHandler({
  async upgrade(request) {
    const headers = new Headers();
    const url = new URL(request.url);
    const token = url.searchParams.get("token");
    headers.set("cookie", `mnemo.session_token=${token}`);

    const user = await requireUser(headers);

    return { namespace: `users/${user.id}/crdt`, context: { token } };
  },

  async open(peer) {
    peer.subscribe(peer.namespace);

    const crdt = await blob.get(peer.namespace);
    if (crdt?.size) peer.send(import.meta.dev ? crdt : await crdt.bytes());
  },

  async message(peer, message) {
    const bytes = message.uint8Array();

    peer.publish(peer.namespace, import.meta.dev ? message.blob() : bytes);

    const crdt = await blob.get(peer.namespace);
    const doc = crdt?.size ? LoroDoc.fromSnapshot(await crdt.bytes()) : new LoroDoc();

    doc.import(bytes);

    const snapshot = doc.export({ mode: "snapshot" });
    await blob.put(peer.namespace, snapshot);

    const headers = new Headers();
    const { token } = peer.context;
    headers.set("cookie", `mnemo.session_token=${token}`);

    const auth = serverAuth();

    await auth.api.ingestion({
      headers,
      body: {
        event: "crdt-sync",
        metadata: { bytes: bytes.length },
      },
    });
  },

  async close(peer) {
    peer.unsubscribe(peer.namespace);
  },
});
