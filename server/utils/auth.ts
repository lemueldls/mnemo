import { betterAuth } from "better-auth";

import { D1Dialect } from "@atinux/kysely-d1";
import type { D1Database } from "@cloudflare/workers-types";

// import {} from "better-auth/plugins";
import { tauri } from "@daveyplate/better-auth-tauri/plugin";

let _auth: ReturnType<typeof betterAuth>;
export function serverAuth() {
  const { github } = useRuntimeConfig().oauth;

  _auth ||= betterAuth({
    database: {
      dialect: new D1Dialect({ database: hubDatabase() as D1Database }),
      type: "sqlite",
    },
    secondaryStorage: {
      get: (key) => hubKV().getItemRaw(`_auth:${key}`),
      set: (key, value, ttl) => {
        return hubKV().set(`_auth:${key}`, value, { ttl });
      },
      delete: (key) => hubKV().del(`_auth:${key}`),
    },
    baseURL: getBaseURL(),
    socialProviders: {
      github: {
        clientId: github.clientId,
        clientSecret: github.clientSecret,
      },
    },
    plugins: [tauri({ scheme: "mnemo" })],
  });

  return _auth;
}

function getBaseURL() {
  const { apiBaseUrl } = useRuntimeConfig().public;
  const url = apiBaseUrl ? new URL(apiBaseUrl) : getRequestURL(useEvent());
  const baseURL = url.origin;

  return baseURL;
}
