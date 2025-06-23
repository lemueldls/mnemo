import type { MaterialSymbol } from "material-symbols";

export interface Space {
  name: string;
  icon: MaterialSymbol;
  color: string;
  order: number;
}

interface Note {
  id: string;
  // name?: string;
  datetime: [number, number, number, number, number];
}

export type NoteKind = "daily" | "sticky" | "prelude";

export async function useSpaces() {
  return await useStorageItem<{ [id: string]: Space }>("spaces.json", {});
}
