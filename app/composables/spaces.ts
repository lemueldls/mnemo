import type { MaterialSymbol } from "material-symbols";

export interface Space {
  name: string;
  icon?: MaterialSymbol;
  color: string;
  order: number;
}

export async function useSpaces() {
  return await useStorageItem<{ [id: string]: Space }>("spaces.json", {});
}
