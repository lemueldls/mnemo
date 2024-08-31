import { object, string } from "valibot";

export default defineEventHandler(async () => {
  const { key, value } = await readBodyWithSchema(
    object({ key: string(), value: string() })
  );

  const userStorage = await useUserStorage();
  await userStorage.setItem(key, value);
});
