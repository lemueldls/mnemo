import type { ThemeKeys, Theme } from "../types";

export const m3ThemeKey = Symbol("theme");

export function useMaterialTheme<C extends ThemeKeys>() {
  return inject<Ref<Theme<C>>>(m3ThemeKey);
}
