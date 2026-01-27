import type { MaterialSymbol } from "material-symbols";

export interface Space {
  name: string;
  icon?: MaterialSymbol;
  color: string;
  order: number;
  archived?: boolean;
}

export async function useSpaces() {
  const spaces = await useStorageMap<Record<string, Space>>("spaces.json");

  return extendRef(spaces, {
    // withArchived() {}
  });
}

export async function useArchivedSpaces() {
  return await useStorageMap<Record<string, Space>>("archived.json");
}
