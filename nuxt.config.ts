// import wasm from "vite-plugin-wasm";

const defaultLocale = "en";
const locales = [
  { code: "en", language: "en-US", name: "English", file: "en-US.json" },
];

// const isDev = process.env.NODE_ENV === "development";
const platform: string = import.meta.env.TAURI_ENV_PLATFORM;

// const internalHost = process.env.TAURI_DEV_HOST || "localhost";

const siteUrl = platform ? "https://tauri.localhost" : "http://localhost:3000";
// const apiBaseUrl = new URL(import.meta.env.NUXT_PUBLIC_API_BASE_URL || siteUrl);

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    "@nuxthub/core",
    "@nuxt/eslint",
    "@nuxtjs/seo",
    "@nuxtjs/i18n",
    "@nuxtjs/color-mode",
    "@vueuse/nuxt",
    "@unocss/nuxt",
    "nuxt-ssr-lit",
    // "nuxt-auth-utils",
    "reka-ui/nuxt",
  ],
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
  // devServer: { https: true },
  vue: { compilerOptions: { isCustomElement: (tag) => tag.startsWith("md-") } },
  runtimeConfig: {
    public: {
      platform,
      apiBaseUrl: "",
      auth: {
        redirectUserTo: "/user",
        redirectGuestTo: "/",
      },
    },
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
  routeRules: {
    "/": { ssr: false },
    "/**": { ssr: false },
    "/space": { ssr: false },
    "/space/**": { ssr: false },
  },
  // devServer: { https: true },
  future: { compatibilityVersion: 4 },
  experimental: {
    typedPages: true,
    viewTransition: true,
    asyncContext: true,
  },
  compatibilityDate: "2025-06-03",
  nitro: {
    esbuild: { options: { target: "esnext" } },
    // moduleSideEffects: ["@material/web"],
    prerender: {
      routes: ["/", "/calendar", "/space"],
      crawlLinks: true,
    },
    experimental: { openAPI: true, websocket: true },
  },
  hub: {
    // analytics: true,
    // remote: true,
    // ai: true,
    // blob: true,
    cache: true,
    database: true,
    kv: true,
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
