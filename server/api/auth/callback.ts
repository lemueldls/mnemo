import { object, string } from "valibot";

export default defineEventHandler(async (event) => {
  const query = await validatedQuery(event, object({ redirect: string() }));
  const token = getCookie(
    event,
    import.meta.dev ? "mnemo.session_token" : "__Secure-mnemo.session_token",
  );

  if (!token)
    throw createError({ statusCode: 401, statusMessage: "Unauthorized" });

  const url = new URL(
    `/auth/confirm?token=${encodeURIComponent(token)}`,
    query.redirect,
  );

  await sendRedirect(event, url.href);
});
