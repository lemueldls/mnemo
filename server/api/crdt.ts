export default defineWebSocketHandler({
  async upgrade(request) {
    const user = await requireUser(request.headers);

    return { context: { key: `users:${user.id}:crdt` } };
  },

  async open(peer) {
    peer.subscribe(peer.context.key as string);

    if (await hubKV().hasItem(peer.context.key as string)) {
      const bytes = await hubKV().getItemRaw(peer.context.key as string);
      peer.send(bytes);
    }
  },

  async message(peer, message) {
    const bytes = message.uint8Array();

    peer.publish(peer.context.key as string, bytes);
    await hubKV().setItemRaw(peer.context.key as string, bytes);
  },

  async close(peer) {
    peer.unsubscribe(peer.context.key as string);
  },
});
