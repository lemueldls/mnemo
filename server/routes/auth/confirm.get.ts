export default defineEventHandler(async (event) => {
  const sessionConfig = useRuntimeConfig().session;
  const session = getCookie(event, sessionConfig.name || "nuxt-session");

  if (!session)
    throw createError({ statusCode: 401, statusMessage: "Unauthorized" });

  await sendRedirect(
    event,
    `/auth/login?session=${encodeURIComponent(session)}`,
  );
});
