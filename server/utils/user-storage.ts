import { prefixStorage } from "unstorage";

import type { H3Event } from "h3";

export async function useUserStorage(event: H3Event) {
  await setUserSession(event, {
    user: { id: "k1xhwjz6xaivhn1tpg6j4osr" },
  });

  const { user } = await getUserSession(event);
  if (!user) throw createError({ status: 401 });

  return prefixStorage(hubKV(), `users:${user.id}`);
}
