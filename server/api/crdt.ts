export default defineWebSocketHandler({
  async upgrade(request) {
    const user = await requireUser(request.headers);

    return { namespace: `users/${user.id}/crdt` };
  },

  async open(peer) {
    peer.subscribe(peer.namespace);

    const bytes = await hubBlob().get(peer.namespace);
    if (bytes?.size) peer.send(bytes);
  },

  async message(peer, message) {
    peer.publish(peer.namespace, message.uint8Array());
    await hubBlob().put(peer.namespace, message.blob());
  },

  async close(peer) {
    peer.unsubscribe(peer.namespace);
  },
});
