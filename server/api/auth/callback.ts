import { object, string } from "valibot";
import { validatedQuery } from "~~/server/utils/schema";

export default defineEventHandler(async (event) => {
  const query = await validatedQuery(event, object({ redirect: string() }));
  const token = getCookie(event, "mnemo.session_token");

  console.log({ query, session: token });

  if (!token)
    throw createError({ statusCode: 401, statusMessage: "Unauthorized" });

  const url = new URL(
    `/auth/confirm?token=${encodeURIComponent(token)}`,
    query.redirect,
  );

  await sendRedirect(event, url.href);
});
