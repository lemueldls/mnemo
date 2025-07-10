export default defineEventHandler(async (event) => {
  const query = getQuery<{ redirect: string }>(event);
  const token = getCookie(event, "mnemo.session_token");

  if (!token)
    throw createError({ statusCode: 401, statusMessage: "Unauthorized" });

  const url = new URL(
    `/auth/confirm?token=${encodeURIComponent(token)}`,
    query.redirect,
  );

  await sendRedirect(event, url.href);
});
