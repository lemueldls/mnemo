import type { StickyNote } from ".";
import { invoke } from "@tauri-apps/api/core";

export async function listStickyNotes(spaceId: string) {
  const notes = await invoke<StickyNote[]>("list_sticky_notes", { spaceId });

  const item = await useStorageItem<StickyNote[]>(
    `spaces/${spaceId}/sticky`,
    [],
  );
  item.value = notes;

  return notes;
}

export async function newStickyNote(spaceId: string) {
  return await invoke<string>("new_sticky_note", { spaceId });
}

export function renameStickyNote(
  spaceId: string,
  noteId: string,
  name: string,
) {
  return invoke<void>("rename_sticky_note", { spaceId, noteId, name });
}

export async function updateStickyNote(
  spaceId: string,
  noteId: string,
  x: number,
  y: number,
  width: number,
  height: number,
) {
  return await invoke<void>("update_sticky_note", {
    spaceId,
    noteId,
    x,
    y,
    width,
    height,
  });
}

export function deleteStickyNote(spaceId: string, noteId: string) {
  return invoke<void>("delete_sticky_note", { spaceId, noteId });
}
