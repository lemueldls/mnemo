import { object, string } from "valibot";

export default defineEventHandler(async () => {
  const { key, initialValue } = await readBodyWithSchema(
    object({ key: string(), initialValue: string() })
  );

  const userStorage = await useUserStorage();
  if (!userStorage.hasItem(key)) await userStorage.setItem(key, initialValue);
  return await userStorage.getItem(key);
});
