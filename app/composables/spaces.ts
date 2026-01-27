import type { MaterialSymbol } from "material-symbols";

export interface Space {
  name: string;
  icon?: MaterialSymbol;
  color: string;
  order: number;
  archived?: boolean;
}

export async function useSpaces(archived?: boolean) {
  return useStorageMap<Record<string, Space>>("spaces.json");
}
