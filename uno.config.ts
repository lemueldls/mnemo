import {
  defineConfig,
  presetTypography,
  presetWind3,
  transformerCompileClass,
  transformerDirectives,
  transformerVariantGroup,
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
