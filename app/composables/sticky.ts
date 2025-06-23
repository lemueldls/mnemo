export interface StickyNote {
  id: string;
  title: string;
  x: number;
  y: number;
  rx: number;
  ry: number;
  width: number;
  height: number;
  // datetime: [number, number, number, number, number];
}

export async function listStickyNotes(spaceId: string) {
  const item = await useStorageItem<StickyNote[]>(
    `spaces/${spaceId}/sticky/notes.json`,
    [],
  );

  return item.value;
}
