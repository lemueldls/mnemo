import {
  MaterialDynamicColors,
  SchemeTonalSpot,
  Hct,
  argbFromHex,
  rgbaFromArgb,
  Blend,
} from "@material/material-color-utilities";

import type { ThemeKeys, Theme } from "../types";

const dynamicColors = Object.keys(MaterialDynamicColors).filter(
  (key) => key !== "contentAccentToneDelta" && key !== "highestSurface",
) as ThemeKeys[];

export function createTheme<K extends ThemeKeys>(
  color: MaybeRefOrGetter<string>,
  dark: MaybeRefOrGetter<boolean>,
  harmonize?: MaybeRefOrGetter<string>,
  keys: K[] = dynamicColors as K[],
): Theme<K> {
  const palette = shallowReactive({}) as Theme<K>["palette"];

  watchEffect(() => {
    const argbColor = argbFromHex(toValue(color));
    const argbHarmonize = harmonize && argbFromHex(toValue(harmonize));

    const scheme = new SchemeTonalSpot(
      Hct.fromInt(argbColor),
      toValue(dark),
      0,
    );

    for (const key of keys) {
      const color = MaterialDynamicColors[key];
      const designColor = color.getArgb(scheme);

      palette[key] = rgbaFromArgb(
        argbHarmonize
          ? Blend.harmonize(designColor, argbHarmonize)
          : designColor,
      );
    }
  });

  return { source: toRef(color), palette };
}
