import { IncomingRequestCfProperties } from "@cloudflare/workers-types";
import { checkout, polar, portal, usage } from "@polar-sh/better-auth";
import { Polar } from "@polar-sh/sdk";
import { betterAuth, BetterAuthPlugin } from "better-auth";
import { DB, drizzleAdapter } from "better-auth/adapters/drizzle";

import * as schema from "../../db/schema";

import { bearer } from "./bearer";

import type { KVStorage } from "hub:kv";
import type { NitroRuntimeConfig } from "nitropack/types";

export interface AuthOptions {
  baseURL?: string;
  db?: DB;
  kv?: KVStorage;
  runtimeConfig: NitroRuntimeConfig;
  polarClient: Polar;
  cf?: IncomingRequestCfProperties;
}

export function createAuth(options?: AuthOptions) {
  const plugins: BetterAuthPlugin[] = [bearer()];

  if (options?.polarClient)
    plugins.push(
      polar({
        client: options?.polarClient,
        createCustomerOnSignUp: true,
        use: [
          checkout({
            products: [{ productId: "ccd6d053-8c91-464f-be19-1644ac837e14", slug: "sync" }],
            authenticatedUsersOnly: true,
          }),
          portal(),
          usage(),
        ],
      }),
    );

  const db = options?.db;
  const kv = options?.kv;
  const github = options?.runtimeConfig?.oauth?.github;

  return betterAuth({
    appName: "Mnemo",
    baseURL: options?.baseURL,
    secret: options?.runtimeConfig?.session?.password,
    database: drizzleAdapter(db || {}, {
      schema,
      provider: "sqlite",
      usePlural: true,
    }),
    secondaryStorage: {
      async get(key) {
        if (!kv) throw createError("Missing KV");
        const hasItem = await kv.hasItem(`_auth:${key}`);

        return hasItem ? JSON.stringify(await kv.getItem(`_auth:${key}`)) : null;
      },
      async set(key, value, ttl) {
        if (!kv) throw createError("Missing KV");
        await kv.setItem(`_auth:${key}`, value, { ttl });
      },
      async delete(key) {
        if (!kv) throw createError("Missing KV");
        await kv.removeItem(`_auth:${key}`);
      },
    },
    socialProviders: {
      github: github && {
        clientId: github.clientId,
        clientSecret: github.clientSecret,
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
    // oxlint-disable-next-line typescript/no-unsafe-type-assertion
    plugins: plugins as [ReturnType<typeof bearer>, ReturnType<typeof polar>],
    logger: { level: import.meta.dev ? "debug" : "warn" },
  });
}
