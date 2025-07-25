import { globalIgnores } from "eslint/config";
import importPlugin from "eslint-plugin-import-x";

// @ts-check
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt([
  {
    plugins: { import: importPlugin },
    rules: {
      "vue/html-self-closing": "off",
      "vue/no-multiple-template-root": "off",
      "vue/multi-word-component-names": "off",
      "vue/max-attributes-per-line": "off",
      "vue/singleline-html-element-content-newline": "off",
      "vue/no-deprecated-slot-attribute": "off",

      "import/order": [
        "warn",
        {
          groups: [
            "builtin",
            "external",
            "internal",
            "parent",
            "sibling",
            "index",
            "object",
            "type",
          ],
          "newlines-between": "always-and-inside-groups",
          // "newlines-between-types": "always-and-inside-groups",
          consolidateIslands: "inside-groups",
          distinctGroup: true,
          alphabetize: {
            order: "asc",
            orderImportKind: "asc",
            caseInsensitive: true,
          },
          named: { enabled: true, types: "types-last" },
          warnOnUnassignedImports: true,
        },
      ],
    },
  },
  globalIgnores(["platform/*"]),
]);
