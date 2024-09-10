import { createId } from "@paralleldrive/cuid2";
import { loginSchema } from "~~/schemas/login";

export default defineEventHandler(async (event) => {
  const { email, password } = await readBodyWithSchema(event, loginSchema);

  const drizzle = useDrizzle();

  const user = await drizzle
    .select()
    .from(tables.users)
    .where(eq(tables.users.email, email))
    .get();
  if (user)
    throw createError({
      status: 401,
      message: "User with this email already exists",
    });

  // const id = createId();

  // await drizzle
  //   .insert(tables.users)
  //   .values({ id })
  //   .execute();

  await setUserSession(event, {
    user: {
      // ... user data
    },
    loggedInAt: new Date(),
    // Any extra fields
  });
});
