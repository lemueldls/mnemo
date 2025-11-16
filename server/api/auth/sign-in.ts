import { object, string } from "valibot";

export default defineEventHandler(async (event) => {
  const query = await validatedQuery(
    event,
    object({ provider: string(), redirect: string(), platform: string() }),
  );

  const auth = serverAuth();
  const reponse = await auth.api.signInSocial({
    body: {
      provider: query.provider,
      callbackURL: `/api/auth/callback?redirect=${encodeURIComponent(query.redirect)}&platform=${query.platform}`,
    },
  });

  return { url: reponse.url! };
});
