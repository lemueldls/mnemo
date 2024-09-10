import { object, string } from "valibot";

export default defineEventHandler(async (event) => {
  const { base } = await readBodyWithSchema(event, object({ base: string() }));

  const userStorage = await useUserStorage(event);

  return await userStorage.getKeys(base);
});
