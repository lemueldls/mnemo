import {
  defineConfig,
  presetWind3,
  presetWebFonts,
  presetTypography,
  // presetIcons,
  transformerVariantGroup,
  transformerDirectives,
  transformerCompileClass,
} from "unocss";

import { presetM3 } from "./modules/m3/uno.config.ts";

export default defineConfig({
  presets: [
    presetWind3(),
    presetTypography(),
    presetWebFonts({ fonts: { sans: "Roboto" } }),
    // presetIcons({ warn: true }),
    presetM3(),
  ],
  transformers: [
    transformerVariantGroup(),
    transformerDirectives(),
    transformerCompileClass(),
  ],
});
