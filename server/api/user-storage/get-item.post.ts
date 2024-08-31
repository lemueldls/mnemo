import { object, string } from "valibot";

export default defineEventHandler(async () => {
  const { key } = await readBodyWithSchema(object({ key: string() }));

  const userStorage = await useUserStorage();
  return await userStorage.getItem(key);
});
