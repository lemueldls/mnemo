import { prefixStorage } from "unstorage";
import { any, number, object, parse, string } from "valibot";

const StorageItemSchema = object({
  key: string(),
  value: any(),
  updatedAt: number(),
});

export default defineWebSocketHandler({
  async upgrade(request) {
    console.log("[UPGRADE]", { headers: request.headers });
    await requireUser(request.headers);
  },

  async open(peer) {
    console.log("[OPEN]", { headers: peer.request.headers });
    const user = await requireUser(peer.request.headers);
    peer.subscribe(`users:${user.id}`);
  },

  async message(peer, message) {
    const item = parse(StorageItemSchema, message.json());
    const { key, value, updatedAt } = item;

    console.log("[MESSAGE]", { headers: peer.request.headers });
    const user = await requireUser(peer.request.headers);
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
    console.log("[CLOSE]", { headers: peer.request.headers });
    const user = await requireUser(peer.request.headers);
    peer.unsubscribe(`users:${user.id}`);
  },
});
