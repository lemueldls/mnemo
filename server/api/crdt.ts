export default defineWebSocketHandler({
  async upgrade(request) {
    const user = await requireUser(request.headers);

    return { namespace: `/users/${user.id}/crdt` };
  },

  async open(peer) {
    peer.subscribe(peer.namespace);

    const bytes = await hubBlob().get(peer.namespace);
    if (bytes?.size) peer.send(bytes);
  },

  async message(peer, message) {
    const bytes = message.blob();

    peer.publish(peer.namespace, bytes);
    await hubBlob().put(peer.namespace, bytes);
  },

  async close(peer) {
    peer.unsubscribe(peer.namespace);
  },
});
