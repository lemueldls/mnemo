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
    request.context.base = `users:${user.id}`;

    console.log("[request upgrade]", JSON.stringify(request.context));
    console.log(
      "[request cookie]",
      JSON.stringify(request.headers.getSetCookie()),
    );
  },

  async open(peer) {
    console.log("[open]", JSON.stringify(peer.context));
    console.log(
      "[open cookie]",
      JSON.stringify(peer.request.headers.getSetCookie()),
    );

    peer.subscribe(peer.context.base as string);
  },

  async message(peer, message) {
    console.log("[message]", JSON.stringify(peer.context));
    console.log(
      "[message cookie]",
      JSON.stringify(peer.request.headers.getSetCookie()),
    );

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
    console.log("[close]", JSON.stringify(peer.context));
    console.log(
      "[close cookie]",
      JSON.stringify(peer.request.headers.getSetCookie()),
    );

    peer.unsubscribe(peer.context.base as string);
  },
});
