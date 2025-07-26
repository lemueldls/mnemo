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
const platform: string = import.meta.env.TAURI_ENV_PLATFORM;

const remoteProjectType = import.meta.env.REMOTE_PROJECT_TYPE;
const isWorkers = remoteProjectType !== "pages";

// const internalHost = process.env.TAURI_DEV_HOST || "localhost";

const siteUrl = platform
  ? "http://tauri.localhost"
  : isDev
    ? "http://localhost:3000"
    : "https://mnemo.nuxt.dev";
// const apiBaseUrl = new URL(import.meta.env.NUXT_PUBLIC_API_BASE_URL || siteUrl);

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
  devtools: { enabled: !platform },

  app: {
    // pageTransition: { name: "conjure" },
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
  future: { compatibilityVersion: 4 },
  experimental: {
    typedPages: true,
    viewTransition: true,
    asyncContext: true,
  },
  compatibilityDate: "2025-07-17",
  nitro: {
    esbuild: { options: { target: "esnext" } },
    prerender: {
      routes: ["/", "/calendar", "/space"],
      crawlLinks: true,
    },
    experimental: { websocket: isWorkers },
  },
  hub: {
    workers: isWorkers,
    cache: !isDev,
    database: true,
    kv: true,
    blob: true,
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
      { name: "Source Sans", provider: "local", global: true },
      { name: "Source Han Sans CN", provider: "local", global: true },
      // { name: "Source Han Sans JP", provider: "local", global: true },
      // { name: "Source Serif", provider: "local", global: true },
      // { name: "Source Han Serif CN", provider: "local", global: true },
      // { name: "Source Han Serif JP", provider: "local", global: true },
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
