export default defineWebSocketHandler({
  async upgrade(request) {
    await requireUserSession(request);
  },

  async open(peer) {
    const { user } = await requireUserSession(peer);
    const base = `users:${user.id}:crdt`;

    peer.subscribe(base);

    if (await hubKV().hasItem(base)) {
      const bytes = await hubKV().getItemRaw(base);
      peer.send(bytes);
    }
  },

  async message(peer, message) {
    const { user } = await requireUserSession(peer);
    const base = `users:${user.id}:crdt`;
    const bytes = message.uint8Array();

    peer.publish(base, bytes);

    await hubKV().setItemRaw(base, bytes);
  },

  async close(peer) {
    const { user } = await requireUserSession(peer);
    peer.unsubscribe(`users:${user.id}:crdt`);
  },
});
