import { invoke } from "@tauri-apps/api/core";

export interface StickyNote {
  id: string;
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
  // datetime: [number, number, number, number, number];
}

export function listStickyNotes(spaceId: string) {
  return invoke<StickyNote[]>("list_sticky_notes", { spaceId });
}

export function newStickyNote(spaceId: string) {
  return invoke<string>("new_sticky_note", { spaceId });
}

export function renameStickyNote(
  spaceId: string,
  noteId: string,
  name: string,
) {
  return invoke<void>("rename_sticky_note", { spaceId, noteId, name });
}

export function updateStickyNote(
  spaceId: string,
  noteId: string,
  x: number,
  y: number,
  width: number,
  height: number,
) {
  return invoke<void>("update_sticky_note", {
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
