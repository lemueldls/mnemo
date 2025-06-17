import type { MaterialDynamicColors } from "@material/material-color-utilities";

export type Rgba = { r: number; g: number; b: number; a: number };

export type ThemeKeys = Exclude<
  keyof typeof MaterialDynamicColors,
  "prototype" | "contentAccentToneDelta" | "colorSpec" | "highestSurface"
>;

export interface Theme<K extends ThemeKeys = ThemeKeys> {
  source: string;
  palette: { [_ in K & string]: Rgba };
}
