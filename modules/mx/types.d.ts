import type {
  MaterialDynamicColors,
  Rgba,
} from "@material/material-color-utilities";

export type ThemeKeys = Exclude<
  keyof typeof MaterialDynamicColors,
  "prototype" | "contentAccentToneDelta" | "highestSurface"
>;

export interface Theme<K extends ThemeKeys = ThemeKeys> {
  source: string;
  palette: { [_ in K & string]: Rgba };
}
