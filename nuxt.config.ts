import wasm from "vite-plugin-wasm";

const defaultLocale = "en";
const locales = [
  { code: "en", language: "en-US", name: "English", file: "en-US.json" },
];

const platform: string = import.meta.env.TAURI_ENV_PLATFORM;

const siteUrl = platform ? "https://tauri.localhost" : "http://localhost:3000";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  // ssr: false,
  devtools: { enabled: true },
  // devServer: { https: true },
  future: { compatibilityVersion: 4 },
  compatibilityDate: "2024-07-07",
  nitro: {
    esbuild: { options: { target: "esnext" } },
    // moduleSideEffects: ["@material/web"],
    // prerender: { routes: ["/"], ignore: ["/space"] },
    experimental: { openAPI: true },
  },
  vite: {
    // clearScreen: false,
    // envPrefix: ["VITE_", "TAURI_"],
    // build: { target: ["safari15"] },
    // server: {
    //   strictPort: true,
    //   hmr: { protocol: "ws", host: "0.0.0.0", port: 5183 },
    // },
    plugins: [wasm()],
  },
  // devServer: { https: true },
  // vue: { compilerOptions: { isCustomElement: (tag) => tag.startsWith("md-") } },
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
  runtimeConfig: { app: { platform }, public: { apiBaseUrl: "" } },
  imports: {
    dirs: [
      "composables",
      "composables/*/index.{ts,js,mjs,mts}",
      platform
        ? "composables/*/tauri.{ts,js,mjs,mts}"
        : "composables/*/server.{ts,js,mjs,mts}",
    ],
  },
  app: {
    layoutTransition: { name: "conjure" },
    head: { templateParams: { separator: "•" } },
  },
  typescript: { shim: false },
  css: ["@unocss/reset/tailwind.css", "@/assets/scss/main.scss"],
  modules: [
    // "@nuxt/ui",
    "@nuxthub/core",
    "@nuxtjs/i18n",
    "@vueuse/nuxt",
    "@unocss/nuxt",
    "@nuxtjs/supabase",
    "nuxt-ssr-lit",
  ],
  hub: { ai: true, blob: true, cache: true, kv: true },
  i18n: {
    lazy: true,
    defaultLocale,
    locales,
    langDir: "locales",
    baseUrl: siteUrl,
  },
  supabase: {
    types: "~~/supabase/database.d.ts",
    redirectOptions: {
      login: "/login",
      callback: "/confirm",
      exclude: ["/"],
      cookieRedirect: true,
    },
  },
  ssrLit: { litElementPrefix: ["md-"] },
});
