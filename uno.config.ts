import {
  defineConfig,
  presetWind3,
  presetTypography,
  transformerVariantGroup,
  transformerDirectives,
  transformerCompileClass,
} from "unocss";

import { presetMx } from "./modules/mx/uno.config";

export default defineConfig({
  presets: [presetWind3(), presetTypography(), presetMx()],
  transformers: [
    transformerVariantGroup(),
    transformerDirectives(),
    transformerCompileClass(),
  ],
});
