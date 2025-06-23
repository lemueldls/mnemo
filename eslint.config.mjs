// @ts-check
import { createConfigForNuxt } from "@nuxt/eslint-config";

export default createConfigForNuxt({ features: { tooling: true } }).prepend({
  rules: {
    "vue/html-self-closing": "off",
    "vue/no-multiple-template-root": "off",
    "vue/multi-word-component-names": "off",
    "vue/max-attributes-per-line": "off",
    "vue/singleline-html-element-content-newline": "off",
    "vue/no-deprecated-slot-attribute": "off",
    "nuxt/nuxt-config-keys-order": "warn",
    "@typescript-eslint/unified-signatures": "off",
  },
});
