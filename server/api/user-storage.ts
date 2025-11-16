import { prefixStorage } from "unstorage";
import { any, number, object, parse, string } from "valibot";

const StorageItemSchema = object({
  key: string(),
  value: any(),
  updatedAt: number(),
});

export default defineWebSocketHandler({
  async upgrade(request) {
    const headers = new Headers();
    const url = new URL(request.url);
    const token = url.searchParams.get("token");
    headers.set("cookie", `mnemo.session_token=${token}`);

    const user = await requireUser(headers);

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

    const clientMoreRecent =
      !meta?.updatedAt || updatedAt > (meta.updatedAt as number);

    if (clientMoreRecent && value !== null && value !== undefined) {
      await userStorage.setItem(key, value);
      await userStorage.setMeta(key, { updatedAt });

      peer.publish(peer.namespace, item);
    } else if (meta)
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
