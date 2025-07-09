import { prefixStorage } from "unstorage";
import { any, number, object, parse, string } from "valibot";

const StorageItemSchema = object({
  key: string(),
  value: any(),
  updatedAt: number(),
});

export default defineWebSocketHandler({
  async upgrade(request) {
    await requireUserSession(request);
  },

  async open(peer) {
    const { user } = await requireUserSession(peer);
    peer.subscribe(`users:${user.id}`);
  },

  async message(peer, message) {
    const item = parse(StorageItemSchema, message.json());
    const { key, value, updatedAt } = item;

    const { user } = await requireUserSession(peer);
    const userStorage = prefixStorage(hubKV(), `users:${user.id}`);

    const hasItem = await userStorage.hasItem(key);
    const meta = hasItem ? await userStorage.getMeta(key) : undefined;

    if (!meta?.updatedAt || updatedAt > (meta.updatedAt as number)) {
      await userStorage.setItem(key, value);
      await userStorage.setMeta(key, { updatedAt });

      peer.publish(`users:${user.id}`, item);
    } else
      peer.send({
        key,
        value: await userStorage.getItem(key),
        updatedAt: meta.updatedAt,
      });
  },

  async close(peer) {
    const { user } = await requireUserSession(peer);
    peer.unsubscribe(`users:${user.id}`);
  },
});
