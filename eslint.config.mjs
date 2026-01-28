import oxlint from "eslint-plugin-oxlint";

import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt(...oxlint.configs["flat/recommended"]);
