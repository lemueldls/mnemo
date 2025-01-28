import { boolean, number, object, string, union } from "valibot";

export default defineEventHandler(async (event) => {
  // const { key, value } = await readBodyWithSchema(
  //   object({ key: string(), value: union([string(), number(), boolean()]) })
  // );
  const { key, value } = await readBody(event);

  const userStorage = await useUserStorage(event);
  await userStorage.setItem(key, value);
  await userStorage.setMeta(key, { updatedAt: Date.now() });
});
