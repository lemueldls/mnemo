import { prefixStorage } from "unstorage";
import { any, number, object, parse, string } from "valibot";

const StorageItemSchema = object({
  key: string(),
  value: any(),
  updatedAt: number(),
});

export default defineWebSocketHandler({
  async upgrade(request) {
    const { headers, context } = request;

    const auth = serverAuth();

    const session = await auth.api.getSession({ headers });
    if (!session)
      throw createError({ statusCode: 401, statusMessage: "Unauthorized" });

    context.base = `users:${session.user.id}`;
  },

  async open(peer) {
    peer.subscribe(peer.context.base as string);
  },

  async message(peer, message) {
    const item = parse(StorageItemSchema, message.json());
    const { key, value, updatedAt } = item;

    const userStorage = prefixStorage(hubKV(), peer.context.base as string);

    const hasItem = await userStorage.hasItem(key);
    const meta = hasItem ? await userStorage.getMeta(key) : undefined;

    if (!meta?.updatedAt || updatedAt > (meta.updatedAt as number)) {
      await userStorage.setItem(key, value);
      await userStorage.setMeta(key, { updatedAt });

      peer.publish(peer.context.base as string, item);
    } else
      peer.send({
        key,
        value: await userStorage.getItem(key),
        updatedAt: meta.updatedAt,
      });
  },

  async close(peer) {
    peer.unsubscribe(peer.context.base as string);
  },
});
