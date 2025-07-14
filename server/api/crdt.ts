export default defineWebSocketHandler({
  async upgrade(request) {
    await requireUser(request.headers);
  },

  async open(peer) {
    const user = await requireUser(peer.request.headers);
    const key = `users:${user.id}:crdt`;

    peer.subscribe(key);

    if (await hubKV().hasItem(key)) {
      const bytes = await hubKV().getItemRaw(key);
      peer.send(bytes);
    }
  },

  async message(peer, message) {
    const user = await requireUser(peer.request.headers);
    const key = `users:${user.id}:crdt`;
    const bytes = message.uint8Array();

    peer.publish(key, bytes);

    await hubKV().setItemRaw(key, bytes);
  },

  async close(peer) {
    const user = await requireUser(peer.request.headers);
    peer.unsubscribe(`users:${user.id}:crdt`);
  },
});
