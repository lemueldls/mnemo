import type { StickyNote } from ".";

export async function listStickyNotes(spaceId: string) {
  const item = await useStorageItem<StickyNote[]>(
    `spaces/${spaceId}/sticky`,
    []
  );

  return item.value;
}
