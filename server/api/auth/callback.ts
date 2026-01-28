import { object, string } from "valibot";

export default defineEventHandler(async (event) => {
  const query = await validatedQuery(event, object({ redirect: string(), platform: string() }));

  const token = getCookie(event, "mnemo.session_token");

  console.log({ token }, token && encodeURIComponent(token), event.headers);

  if (!token) throw createError({ statusCode: 401, statusMessage: "Unauthorized" });

  const url = new URL(
    `/auth/confirm?token=${btoa(token)}&redirect=${encodeURIComponent(query.redirect)}&platform=${query.platform}`,
    getBaseURL(),
  );

  await sendRedirect(event, url.href);
});
