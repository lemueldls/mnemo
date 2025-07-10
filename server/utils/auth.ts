import { betterAuth } from "better-auth";

import { D1Dialect } from "@atinux/kysely-d1";
import { D1Database } from "@cloudflare/workers-types";

import { tauri } from "@daveyplate/better-auth-tauri/plugin";

const runtimeConfig = useRuntimeConfig();

export async function requireUser(headers: Headers) {
  const auth = serverAuth();

  const session = await auth.api.getSession({ headers });
  if (!session)
    throw createError({ statusCode: 401, statusMessage: "Unauthorized" });

  return session.user;
}

let _auth: ReturnType<typeof betterAuth>;

export function serverAuth() {
  const { password } = runtimeConfig.session;
  const { github } = runtimeConfig.oauth;

  _auth ||= betterAuth({
    appName: "Mnemo",
    baseURL: getBaseURL(),
    secret: password || undefined,
    database: {
      dialect: new D1Dialect({
        database: hubDatabase() as D1Database,
      }),
      type: "sqlite",
    },
    secondaryStorage: {
      async get(key) {
        return (await hubKV().hasItem(`_auth:${key}`))
          ? JSON.stringify(await hubKV().getItem(`_auth:${key}`))
          : null;
      },
      async set(key, value, ttl) {
        await hubKV().setItem(`_auth:${key}`, value, { ttl });
      },
      async delete(key) {
        await hubKV().removeItem(`_auth:${key}`);
      },
    },
    session: {
      expiresIn: 60 * 60 * 24 * 7 * 4 * 4, // 4 months
    },
    socialProviders: {
      github: {
        clientId: github.clientId!,
        clientSecret: github.clientSecret!,
        redirectURI: github.redirectURL,
      },
    },
    plugins: [tauri({ scheme: "mnemo" })],
    advanced: {
      cookiePrefix: "mnemo",
      cookies: {
        session_token: {
          name: "session_token",
          attributes: import.meta.dev
            ? { sameSite: "lax", secure: false, httpOnly: false }
            : { sameSite: "none", secure: true, httpOnly: true },
        },
      },
    },
  });

  return _auth;
}

function getBaseURL() {
  const { apiBaseUrl } = runtimeConfig.public;

  const url = apiBaseUrl ? new URL(apiBaseUrl) : getRequestURL(useEvent());
  const baseURL = url.origin;

  return baseURL;
}
