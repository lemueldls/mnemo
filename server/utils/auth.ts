import { D1Dialect } from "@atinux/kysely-d1";
import { checkout, polar, portal, usage } from "@polar-sh/better-auth";
import { Polar } from "@polar-sh/sdk";
import { betterAuth } from "better-auth";

import type { D1Database } from "@cloudflare/workers-types";

const runtimeConfig = useRuntimeConfig();

const polarClient = new Polar({ accessToken: runtimeConfig.polar.accessToken });

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
    trustedOrigins: [
      "http://localhost:3000",
      "http://tauri.localhost",
      "https://mnemo.nuxt.dev",
      "https://mnemo.pages.dev",
      "https://notes.lemueldls.dev",
      "https://notes.workers.lemueldls.dev",
    ],
    database: {
      dialect: new D1Dialect({
        database: hubDatabase() as D1Database,
      }),
      type: "sqlite",
    },
    secondaryStorage: {
      async get(key) {
        const hasItem = await hubKV().hasItem(`_auth:${key}`);

        return hasItem
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
    socialProviders: {
      github: {
        clientId: github.clientId!,
        clientSecret: github.clientSecret!,
        redirectURI: github.redirectURL,
      },
    },
    advanced: {
      cookiePrefix: "mnemo",
      useSecureCookies: false,
      defaultCookieAttributes: import.meta.dev
        ? { sameSite: "lax", secure: false, httpOnly: false }
        : { sameSite: "none", secure: true, httpOnly: false },
    },
    plugins: [
      polar({
        client: polarClient,
        createCustomerOnSignUp: true,
        use: [
          checkout({
            products: [
              {
                productId: "ccd6d053-8c91-464f-be19-1644ac837e14",
                slug: "sync",
              },
            ],
            authenticatedUsersOnly: true,
          }),
          portal(),
          usage(),
        ],
      }),
    ],
  });

  return _auth;
}

export function getBaseURL() {
  const { apiBaseUrl } = runtimeConfig.public;
  const url = apiBaseUrl ? new URL(apiBaseUrl) : getRequestURL(useEvent());

  return url.origin;
}
