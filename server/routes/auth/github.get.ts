import { createId } from "@paralleldrive/cuid2";
import type { H3Event } from "h3";

export default oauthGitHubEventHandler({
  config: { emailRequired: true },
  async onSuccess(event, { user, tokens }) {
    const { email, name } = user;
    await login(event, email, name);

    return sendRedirect(event, "/");
  },
  // onError(event, error) {
  //   console.error("GitHub OAuth error:", error);
  //   return sendRedirect(event, "/");
  // },
});

async function login(event: H3Event, email: string, name: string) {
  const drizzle = useDrizzle();

  const user = await drizzle
    .select()
    .from(tables.users)
    .where(eq(tables.users.email, email))
    .get();

  if (user) {
    const { id } = user;
    await setUserSession(event, { user: { id } });

    return;
  }

  const id = createId();

  const date = new Date();
  const now_utc = Date.UTC(
    date.getUTCFullYear(),
    date.getUTCMonth(),
    date.getUTCDate(),
    date.getUTCHours(),
    date.getUTCMinutes(),
    date.getUTCSeconds()
  );

  const createdAt = new Date(now_utc);

  await drizzle.insert(tables.users).values({ id, email, createdAt }).execute();
  await setUserSession(event, { user: { id } });
}
