import {
  Blend,
  Hct,
  MaterialDynamicColors,
  SchemeTonalSpot,
  alphaFromArgb,
  argbFromHex,
  blueFromArgb,
  greenFromArgb,
  redFromArgb,
} from "@material/material-color-utilities";

import type { Theme, ThemeKeys } from "../types";

const dynamicColors = Object.keys(MaterialDynamicColors).filter(
  (key) => key !== "contentAccentToneDelta" && key !== "colorSpec" && key !== "highestSurface",
) as ThemeKeys[];

export const createTheme = useMemoize(
  <K extends ThemeKeys>(
    source: string,
    dark: boolean,
    harmonize?: string,
    keys: K[] = dynamicColors as K[],
  ): Theme<K> => {
    const palette = {} as Theme<K>["palette"];

    watchEffect(() => {
      const argbColor = argbFromHex(source);
      const argbHarmonize = harmonize && argbFromHex(harmonize);

      const scheme = new SchemeTonalSpot(Hct.fromInt(argbColor), dark, 0, "2025");

      for (const key of keys) {
        const color = MaterialDynamicColors[key];
        const designColor = color.getArgb(scheme);

        const argb = argbHarmonize ? Blend.harmonize(designColor, argbHarmonize) : designColor;

        palette[key] = {
          r: redFromArgb(argb),
          g: greenFromArgb(argb),
          b: blueFromArgb(argb),
          a: alphaFromArgb(argb),
        };
      }
    });

    return { source, palette };
  },
);
