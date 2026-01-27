import type { MaterialDynamicColors } from "@material/material-color-utilities";

export interface Rgba {
  r: number;
  g: number;
  b: number;
  a: number;
}

export type ThemeKeys = Exclude<
  keyof typeof MaterialDynamicColors,
  "prototype" | "contentAccentToneDelta" | "colorSpec" | "highestSurface"
>;

export interface Theme<K extends ThemeKeys = ThemeKeys> {
  source: string;
  palette: Record<K & string, Rgba>;
}
