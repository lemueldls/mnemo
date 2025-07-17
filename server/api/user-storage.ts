import { prefixStorage } from "unstorage";
import { any, number, object, parse, string } from "valibot";

const StorageItemSchema = object({
  key: string(),
  value: any(),
  updatedAt: number(),
});

export default defineWebSocketHandler({
  async upgrade(request) {
    const user = await requireUser(request.headers);

    return { namespace: `users:${user.id}` };
  },

  async open(peer) {
    peer.subscribe(peer.namespace);
  },

  async message(peer, message) {
    const item = parse(StorageItemSchema, message.json());
    const { key, value, updatedAt } = item;

    const userStorage = prefixStorage(hubKV(), peer.namespace);

    const hasItem = await userStorage.hasItem(key);
    const meta = hasItem ? await userStorage.getMeta(key) : undefined;

    if (!meta?.updatedAt || updatedAt > (meta.updatedAt as number)) {
      await userStorage.setItem(key, value);
      await userStorage.setMeta(key, { updatedAt });

      peer.publish(peer.namespace, item);
    } else
      peer.send({
        key,
        value: await userStorage.getItem(key),
        updatedAt: meta.updatedAt,
      });
  },

  async close(peer) {
    peer.unsubscribe(peer.namespace);
  },
});
