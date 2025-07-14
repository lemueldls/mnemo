import { prefixStorage } from "unstorage";
import { any, number, object, parse, string } from "valibot";

const StorageItemSchema = object({
  key: string(),
  value: any(),
  updatedAt: number(),
});

export default defineWebSocketHandler({
  async upgrade(request) {
    console.log("[upgrade context]", JSON.stringify(request.context));
    console.log(
      "[upgrade headers]",
      request.headers
        ? JSON.stringify(Object.fromEntries(request.headers.entries()))
        : null,
    );
    const user = await requireUser(request.headers);

    console.log("[upgrade user]", JSON.stringify(user));

    return { context: { base: `users:${user.id}` } };
  },

  async open(peer) {
    console.log("[open context]", JSON.stringify(peer.context));
    console.log(
      "[open headers]",
      peer.request.headers
        ? JSON.stringify(Object.fromEntries(peer.request.headers.entries()))
        : null,
    );

    const user = await requireUser(peer.request.headers);
    console.log("[open user]", JSON.stringify(user));

    peer.subscribe(`users:${user.id}`);
  },

  async message(peer, message) {
    console.log("[message context]", JSON.stringify(peer.context));
    console.log(
      "[message headers]",
      peer.request.headers
        ? JSON.stringify(Object.fromEntries(peer.request.headers.entries()))
        : null,
    );

    const user = await requireUser(peer.request.headers);
    console.log("[message user]", JSON.stringify(user));
    const base = `users:${user.id}`;

    const item = parse(StorageItemSchema, message.json());
    const { key, value, updatedAt } = item;

    const userStorage = prefixStorage(hubKV(), base);

    const hasItem = await userStorage.hasItem(key);
    const meta = hasItem ? await userStorage.getMeta(key) : undefined;

    if (!meta?.updatedAt || updatedAt > (meta.updatedAt as number)) {
      await userStorage.setItem(key, value);
      await userStorage.setMeta(key, { updatedAt });

      peer.publish(base, item);
    } else
      peer.send({
        key,
        value: await userStorage.getItem(key),
        updatedAt: meta.updatedAt,
      });
  },

  async close(peer) {
    console.log("[close context]", JSON.stringify(peer.context));
    console.log(
      "[close headers]",
      peer.request.headers
        ? JSON.stringify(Object.fromEntries(peer.request.headers.entries()))
        : null,
    );

    const user = await requireUser(peer.request.headers);
    console.log("[close user]", JSON.stringify(user));
    peer.unsubscribe(`users:${user.id}`);
  },
});
