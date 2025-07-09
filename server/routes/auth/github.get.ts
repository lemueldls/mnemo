import { createId } from "@paralleldrive/cuid2";

import type { H3Event } from "h3";

export default defineOAuthGitHubEventHandler({
  config: { emailRequired: true },
  async onSuccess(event, { user }) {
    if (!user.email) throw createError("Email is required");
    await login(event, user.email);

    return sendRedirect(event, "/auth/confirm");
  },
  onError(_event, error) {
    throw createError(error);
  },
});

async function login(event: H3Event, email: string) {
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
  const createdAt = new Date(Date.now());

  await drizzle.insert(tables.users).values({ id, email, createdAt }).execute();

  await setUserSession(event, { user: { id } });
}
