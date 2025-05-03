// @ts-check
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt({
  rules: {
    // "vue/html-self-closing": "off",
    "vue/no-multiple-template-root": "off",
    "vue/multi-word-component-names": "off",
    "vue/max-attributes-per-line": ["warn", { singleline: 3 }],
    "vue/singleline-html-element-content-newline": "off",
    "nuxt/nuxt-config-keys-order": "warn",
    // "@stylistic/arrow-parens": ["warn", "always"],
  },
});
