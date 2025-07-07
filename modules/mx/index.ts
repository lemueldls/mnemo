import {
  addComponentsDir,
  addImportsDir,
  createResolver,
  defineNuxtModule,
} from "nuxt/kit";

export default defineNuxtModule({
  meta: { name: "mx" },
  setup() {
    const { resolve } = createResolver(import.meta.url);

    addImportsDir(resolve("./composables"));

    addComponentsDir({
      prefix: "mx",
      pathPrefix: false,
      path: resolve("./components"),
      ignore: ["**/abstract.vue"],
    });
  },
});
