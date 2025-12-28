import { checkout, polar, portal, usage } from "@polar-sh/better-auth";
import { Polar } from "@polar-sh/sdk";
import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { bearer } from "better-auth/plugins/bearer";

import { createSharedComposable } from "@vueuse/core";

const runtimeConfig = useRuntimeConfig();
const { password } = runtimeConfig.session;
const { github } = runtimeConfig.oauth;
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

export const serverAuth = createSharedComposable(() =>
  betterAuth({
    appName: "Mnemo",
    baseURL: getBaseURL(),
    secret: password || undefined,
    database: drizzleAdapter(db, {
      schema,
      provider: "sqlite",
      usePlural: true,
    }),
    secondaryStorage: {
      async get(key) {
        const hasItem = await kv.hasItem(`_auth:${key}`);

        return hasItem ? JSON.stringify(await kv.getItem(`_auth:${key}`)) : null;
      },
      async set(key, value, ttl) {
        await kv.setItem(`_auth:${key}`, value, { ttl });
      },
      async delete(key) {
        await kv.removeItem(`_auth:${key}`);
      },
    },
    socialProviders: {
      github: {
        clientId: github.clientId!,
        clientSecret: github.clientSecret!,
        redirectURI: github.redirectURL,
      },
    },
    account: { skipStateCookieCheck: true },
    advanced: {
      cookiePrefix: "mnemo",
      useSecureCookies: false,
      disableCSRFCheck: true,
      disableOriginCheck: true,
      defaultCookieAttributes: import.meta.dev
        ? { sameSite: "lax", secure: false, httpOnly: true }
        : { sameSite: "none", secure: true, httpOnly: true },
    },
    plugins: [
      bearer(),
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
    ] as const,
    logger: { level: import.meta.dev ? "debug" : "warn" },
  }),
);

export function getBaseURL() {
  const url = apiBaseUrl ? new URL(apiBaseUrl) : getRequestURL(useEvent());

  return url.origin;
}
