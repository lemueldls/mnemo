import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

const defaultLocale = "en";
const locales = [
  { code: "en", dir: "ltr", language: "en-US", file: "en.json" },
  { code: "es", dir: "ltr", language: "es-ES", file: "es.json" },
  { code: "de", dir: "ltr", language: "de-DE", file: "de.json" },
  { code: "he", dir: "rtl", language: "he-IL", file: "he.json" },
  { code: "zh", dir: "ltr", language: "zh-CN", file: "zh.json" },
  { code: "ja", dir: "ltr", language: "ja-JP", file: "ja.json" },
];

const isDev = process.env.NODE_ENV === "development";
const apiBaseUrl = import.meta.env.NUXT_PUBLIC_API_BASE_URL;
const platform: string = import.meta.env.TAURI_ENV_PLATFORM;
// const internalHost = import.meta.env.TAURI_DEV_HOST || "localhost";

const siteUrl = platform
  ? "http://tauri.localhost"
  : isDev
    ? "http://localhost:3000"
    : apiBaseUrl;

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    "@nuxthub/core",
    "@nuxt/eslint",
    "@nuxt/fonts",
    "@nuxtjs/i18n",
    "@nuxtjs/color-mode",
    "@vueuse/nuxt",
    "@unocss/nuxt",
    "reka-ui/nuxt",
  ],
  devtools: {
    enabled: !platform,
    timeline: { enabled: true },
  },
  app: {
    layoutTransition: { name: "conjure" },
    head: {
      templateParams: { separator: "•" },
      meta: [
        {
          name: "viewport",
          content:
            "width=device-width, minimum-scale=1.0, maximum-scale=1.0, viewport-fit=cover",
        },
      ],
    },
  },
  css: ["@unocss/reset/tailwind.css", "@/assets/scss/main.scss"],
  vue: { compilerOptions: { isCustomElement: (tag) => tag.startsWith("md-") } },
  runtimeConfig: {
    public: {
      platform,
      apiBaseUrl: "",
    },
    session: {
      password: "",
    },
    oauth: {
      github: {
        clientId: "",
        clientSecret: "",
        redirectURL: "",
      },
    },
    polar: {
      accessToken: "",
      webhookSecret: "",
    },
  },
  sourcemap: true,
  experimental: {
    typedPages: true,
    viewTransition: true,
    asyncContext: true,
  },
  compatibilityDate: "2025-12-25",
  nitro: {
    preset: "cloudflare-durable",
    cloudflare: { deployConfig: true },
    esbuild: { options: { target: "esnext" } },
    prerender: {
      routes: ["/", "/calendar", "/space"],
      crawlLinks: true,
    },
    experimental: { websocket: true, wasm: true },
  },
  hub: {
    db: { dialect: "sqlite", driver: "d1" },
    kv: { driver: "cloudflare-kv-binding", binding: "KV" },
    cache: { driver: "cloudflare-kv-binding", binding: "CACHE" },
    blob: { driver: "cloudflare-r2", binding: "BLOB" },
  },
  vite: {
    plugins: [topLevelAwait(), wasm()],
  },
  eslint: {
    config: {
      nuxt: { sortConfigKeys: true },
      import: false,
      typescript: true,
      tooling: true,
      stylistic: false,
    },
  },
  fonts: {
    families: [
      { name: "Maple Mono", provider: "local", global: true },
      { name: "Maple Mono CN", provider: "local", global: true },
    ],
  },
  i18n: {
    locales,
    defaultLocale,
    langDir: "locales",
    lazy: true,
    strategy: "no_prefix",
    baseUrl: siteUrl,
  },
});
