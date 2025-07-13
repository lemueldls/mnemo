import { createId } from "@paralleldrive/cuid2";

import type { H3Event } from "h3";
import { object, string } from "valibot";

function createGithubEventHandler(redirectURL?: string) {
  return defineOAuthGitHubEventHandler({
    config: { emailRequired: true, redirectURL },
    async onSuccess(event, { user }) {
      if (!user.email) throw createError("Email is required");
      await login(event, user.email);

      const query = await validatedQuery(
        event,
        object({ redirect: string(), platform: string() }),
      );

      const { apiBaseUrl } = useRuntimeConfig().public;
      const baseUrl = apiBaseUrl
        ? new URL(apiBaseUrl)
        : getRequestURL(useEvent());

      const url = new URL(
        `/auth/confirm?redirect=${encodeURIComponent(query.redirect)}&platform=${query.platform}`,
        baseUrl,
      );

      return sendRedirect(event, url.href);
    },
    onError(_event, error) {
      throw createError(error);
    },
  });
}

export default defineEventHandler(async (event) => {
  const query = await validatedQuery(
    event,
    object({ redirect: string(), platform: string() }),
  );
  const githubEventHandler = createGithubEventHandler(
    `${getRequestURL(event).origin}/auth/github?redirect=${encodeURIComponent(query.redirect)}&platform=${query.platform}`,
  );

  return githubEventHandler(event);
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
