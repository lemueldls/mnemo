import { createSharedComposable } from "@vueuse/core";

import { betterAuth } from "better-auth";
// import { drizzleAdapter } from "better-auth/adapters/drizzle";

import { D1Dialect } from "@atinux/kysely-d1";
import type { D1Database } from "@cloudflare/workers-types";

// import {} from "better-auth/plugins";
import { tauri } from "@daveyplate/better-auth-tauri/plugin";

const { oauth } = useRuntimeConfig();

export const serverAuth = createSharedComposable(() =>
  betterAuth({
    database: {
      dialect: new D1Dialect({ database: hubDatabase() as D1Database }),
      type: "sqlite",
    },
    // database: drizzleAdapter(useDrizzle(), { provider: "sqlite" }),
    secondaryStorage: {
      get: (key) => hubKV().getItemRaw(`_auth:${key}`),
      set: (key, value, ttl) => {
        return hubKV().set(`_auth:${key}`, value, { ttl });
      },
      delete: (key) => hubKV().del(`_auth:${key}`),
    },
    baseURL: getBaseURL(),
    // emailAndPassword: {
    //   enabled: true,
    // },
    socialProviders: {
      github: {
        clientId: oauth.github.clientId,
        clientSecret: oauth.github.clientSecret,
      },
    },
    account: {
      accountLinking: {
        enabled: true,
      },
    },
    plugins: [
      tauri({
        scheme: "mnemo", // Your app's deep link scheme
        callbackURL: "/", // Optional: Where to redirect after auth (default: "/")
        successText: "Authentication successful! You can close this window.", // Optional
        successURL: "/", // Optional: Custom success page URL that will receive a ?tauriRedirect search parameter
        debugLogs: true, // Optional: Enable debug logs
      }),
    ],
  }),
);

function getBaseURL() {
  let baseURL = process.env.BETTER_AUTH_URL;
  if (!baseURL) {
    try {
      baseURL = getRequestURL(useEvent()).origin;
    } catch (e) {
      console.error(e);
    }
  }
  return baseURL;
}
