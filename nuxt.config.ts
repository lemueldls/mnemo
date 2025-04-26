// import wasm from "vite-plugin-wasm";

const defaultLocale = "en";
const locales = [
  { code: "en", language: "en-US", name: "English", file: "en-US.json" },
];

const isDev = process.env.NODE_ENV === "development";
const platform: string = import.meta.env.TAURI_ENV_PLATFORM;

// const internalHost = process.env.TAURI_DEV_HOST || "localhost";

const siteUrl = platform ? "https://tauri.localhost" : "http://localhost:3000";
const apiBaseUrl = new URL(import.meta.env.NUXT_PUBLIC_API_BASE_URL || siteUrl);

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false,
  devtools: { enabled: !platform },
  // devServer: { https: true },
  future: { compatibilityVersion: 4 },
  compatibilityDate: "2024-07-07",
  nitro: {
    esbuild: { options: { target: "esnext" } },
    // moduleSideEffects: ["@material/web"],
    prerender: {
      routes: ["/", "/calendar", "/space"],
      crawlLinks: true,
    },
    experimental: { openAPI: true, websocket: true },
  },
  vite: {
    // clearScreen: false,
    // envPrefix: ["VITE_", "TAURI_"],
    // build: { target: ["safari15"] },
    // server: {
    //   strictPort: true,
    //   hmr: { protocol: "ws", host: "0.0.0.0", port: 5183 },
    // },
    // plugins: [wasm()],
  },
  // devServer: { https: true },
  vue: { compilerOptions: { isCustomElement: (tag) => tag.startsWith("md-") } },
  // routeRules: {
  //   "/**": { ssr: false },
  //   "/splashscreen": { ssr: !!platform, static: false },
  // },
  experimental: {
    typedPages: true,
    componentIslands: true,
    asyncContext: true,
    headNext: true,
    viewTransition: true,
  },
  runtimeConfig: {
    app: { platform },
    public: { platform, apiBaseUrl: "" },
    // session: {
    //   maxAge: 60 * 60 * 24 * 7 * 4 * 4, // 4 months
    //   cookie: isDev
    //     ? { sameSite: "none", secure: true }
    //     : { sameSite: "lax", secure: false },
    // },
    oauth: {
      github: {
        clientId: "",
        clientSecret: "",
        redirectURL: "",
      },
    },
  },
  imports: {
    dirs: [
      "composables",
      "composables/*/index.{ts,js,mjs,mts}",
      // platform
      //   ? "composables/*/tauri.{ts,js,mjs,mts}"
      //   : "composables/*/server.{ts,js,mjs,mts}",
      "composables/*/server.{ts,js,mjs,mts}",
    ],
  },
  app: {
    layoutTransition: { name: "conjure" },
    head: { templateParams: { separator: "•" } },
  },
  css: ["@unocss/reset/tailwind.css", "@/assets/scss/main.scss"],
  modules: [
    "@nuxthub/core",
    "@nuxt/eslint",
    "@nuxtjs/color-mode",
    "@nuxtjs/i18n",
    "@vueuse/nuxt",
    "@unocss/nuxt",
    "nuxt-ssr-lit",
    "nuxt-auth-utils",
    "reka-ui/nuxt",
  ],
  hub: {
    // analytics: true,
    workers: true,
    // remote: true,
    ai: true,
    // blob: true,
    cache: true,
    database: true,
    kv: true,
    vectorize: {},
  },
  i18n: {
    lazy: true,
    defaultLocale,
    locales,
    langDir: "locales",
    baseUrl: siteUrl,
  },
  // ssrLit: { litElementPrefix: ["md-"] },
});
