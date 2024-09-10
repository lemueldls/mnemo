import { object, string } from "valibot";

export default defineEventHandler(async (event) => {
  // const { key, initialValue } = await readBodyWithSchema(
  //     event,
  //   object({ key: string(), initialValue: string() })
  // );
  const { key, initialValue } = await readBody(useEvent());

  const userStorage = await useUserStorage(event);

  console.log({ key, initialValue });

  if (!userStorage.hasItem(key)) {
    await userStorage.setItem(key, initialValue);

    return initialValue;
  }

  return await userStorage.getItem(key);
});
