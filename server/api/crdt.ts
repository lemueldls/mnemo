export default defineWebSocketHandler({
  async upgrade(request) {
    const user = await requireUser(request.headers);

    return { context: { base: `users:${user.id}:crdt` } };
  },

  async open(peer) {
    peer.subscribe(peer.context.base as string);

    if (await hubKV().hasItem(peer.context.base as string)) {
      const bytes = await hubKV().getItemRaw(peer.context.base as string);
      peer.send(bytes);
    }
  },

  async message(peer, message) {
    const bytes = message.uint8Array();

    peer.publish(peer.context.base as string, bytes);
    await hubKV().setItemRaw(peer.context.base as string, bytes);
  },

  async close(peer) {
    peer.unsubscribe(peer.context.base as string);
  },
});
