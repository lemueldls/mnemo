import type { Theme, ThemeKeys } from "../types";

export const mxThemeKey = Symbol("mx-theme");

export function useMaterialTheme<C extends ThemeKeys>() {
  return inject<Ref<Theme<C>>>(mxThemeKey);
}
