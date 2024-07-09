import {
  defineConfig,
  presetUno,
  presetTypography,
  // presetIcons,
  transformerVariantGroup,
  transformerDirectives,
  transformerCompileClass,
} from "unocss";

import { presetM3 } from "./modules/m3/uno.config";

export default defineConfig({
  presets: [
    presetUno(),
    presetTypography(),
    // presetIcons({ warn: true }),
    presetM3(),
  ],
  transformers: [
    transformerVariantGroup(),
    transformerDirectives(),
    transformerCompileClass(),
  ],
});
