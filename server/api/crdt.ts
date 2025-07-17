export default defineWebSocketHandler({
  async upgrade(request) {
    const user = await requireUser(request.headers);

    return { namespace: `users:${user.id}:crdt` };
  },

  async open(peer) {
    peer.subscribe(peer.namespace);

    if (await hubKV().hasItem(peer.namespace)) {
      const bytes = await hubKV().getItemRaw(peer.namespace);
      peer.send(bytes);
    }
  },

  async message(peer, message) {
    const bytes = message.uint8Array();

    peer.publish(peer.namespace, bytes);
    await hubKV().setItemRaw(peer.namespace, bytes);
  },

  async close(peer) {
    peer.unsubscribe(peer.namespace);
  },
});
