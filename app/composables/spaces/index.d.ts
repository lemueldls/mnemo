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
