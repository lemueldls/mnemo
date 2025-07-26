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

    await hubBlob().put(peer.namespace, bytes);
  },

  async close(peer) {
    peer.unsubscribe(peer.namespace);
  },
});
