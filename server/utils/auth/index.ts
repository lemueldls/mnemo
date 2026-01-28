import { Polar } from "@polar-sh/sdk";
import defu from "defu";
import { db } from "hub:db";
import { kv } from "hub:kv";

import { AuthOptions, createAuth } from "./client";

const runtimeConfig = useRuntimeConfig();
const { apiBaseUrl } = runtimeConfig.public;

const polarClient = new Polar({
  accessToken: runtimeConfig.polar.accessToken,
  server: import.meta.dev ? "sandbox" : "production",
});

export async function requireUser(headers: Headers) {
  const auth = serverAuth();

  const session = await auth.api.getSession({ headers });
  if (!session) throw createError({ statusCode: 401, statusMessage: "Unauthorized" });

  return session.user;
}

export const serverAuth = (options?: AuthOptions) =>
  createAuth(
    defu(options, {
      baseURL: getBaseURL(),
      db,
      kv,
      runtimeConfig: useRuntimeConfig(),
      polarClient,
      // cf: useEvent()?.context.cf,
    }),
  );

export function getBaseURL() {
  const url = apiBaseUrl ? new URL(apiBaseUrl) : getRequestURL(useEvent());

  return url.origin;
}
