export default defineWebSocketHandler({
  async upgrade(request) {
    const user = await requireUser(request.headers);

    return { namespace: `users/${user.id}/crdt` };
  },

  async open(peer) {
    peer.subscribe(peer.namespace);

    const bytes = await hubBlob().get(peer.namespace);
    if (bytes?.size) peer.send(await bytes.arrayBuffer());
  },

  async message(peer, message) {
    const buffer = message.arrayBuffer() as ArrayBuffer;

    peer.publish(peer.namespace, buffer);
    await hubBlob().put(peer.namespace, buffer);
  },

  async close(peer) {
    peer.unsubscribe(peer.namespace);
  },
});
