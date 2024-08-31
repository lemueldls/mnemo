import { serverSupabaseUser } from "#supabase/server";
import { prefixStorage } from "unstorage";

export async function useUserStorage() {
  const event = useEvent();

  const user = await serverSupabaseUser(event);
  if (!user) throw createError({ status: 401 });

  return prefixStorage(hubKV(), `users:${user.id}`);
}
