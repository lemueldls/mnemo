import { invoke } from "@tauri-apps/api/core";
import { Store } from "@tauri-apps/plugin-store";
import type { Space } from ".";

export async function listSpaces() {
  return await useStorageItem("spaces", []);
}

export async function readSpaceFile(
  kind: NoteKind,
  spaceId: string,
  path: string
) {
  const file = await useStorageItem(`spaces/${spaceId}/${kind}/${path}`, "");

  return file.value;
}

export async function syncSpaceFile(
  kind: NoteKind,
  spaceId: string,
  path: string,
  text: string
) {
  const file = await useStorageItem(`spaces/${spaceId}/${kind}/${path}`, "");
  file.value = text;
}
