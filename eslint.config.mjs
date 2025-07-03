// @ts-check
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt({
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
