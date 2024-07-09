import {
  createResolver,
  defineNuxtModule,
  addComponentsDir,
  addImportsDir,
} from "nuxt/kit";

export default defineNuxtModule({
  meta: { name: "m3" },
  setup() {
    const { resolve } = createResolver(import.meta.url);

    addImportsDir(resolve("./composables"));

    addComponentsDir({
      prefix: "m3",
      pathPrefix: false,
      path: resolve("./components"),
      ignore: ["**/abstract.vue"],
    });
  },
});
