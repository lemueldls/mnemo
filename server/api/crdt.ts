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
    const user = await requireUser(request.headers);

    return { namespace: `users/${user.id}/crdt` };
  },

  async open(peer) {
    peer.subscribe(peer.namespace);

    const blob = await hubBlob().get(peer.namespace);
    if (blob?.size) peer.send(import.meta.dev ? blob : await blob.bytes());
  },

  async message(peer, message) {
    const bytes = message.uint8Array();

    peer.publish(peer.namespace, import.meta.dev ? message.blob() : bytes);

    const blob = await hubBlob().get(peer.namespace);
    const doc = blob?.size
      ? LoroDoc.fromSnapshot(await blob.bytes())
      : new LoroDoc();

    doc.import(bytes);

    const snapshot = doc.export({ mode: "snapshot" });
    await hubBlob().put(peer.namespace, snapshot);
  },

  async close(peer) {
    peer.unsubscribe(peer.namespace);
  },
});
