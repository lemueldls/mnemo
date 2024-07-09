import type {
  MaterialDynamicColors,
  Rgba,
} from "@material/material-color-utilities";

export type ThemeKeys = Exclude<
  keyof typeof MaterialDynamicColors,
  "prototype" | "contentAccentToneDelta" | "highestSurface"
>;

export interface Theme<K extends ThemeKeys = ThemeKeys> {
  source: Ref<string>;
  palette: UnwrapNestedRefs<{ [_ in K & string]: Rgba }>;
}
