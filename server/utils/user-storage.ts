import type { H3Event } from "h3";
import { prefixStorage } from "unstorage";

export async function useUserStorage(event: H3Event) {
  const { user } = await getUserSession(event);
  if (!user) throw createError({ status: 401 });

  return prefixStorage(hubKV(), `users:${user.id}`);
}
