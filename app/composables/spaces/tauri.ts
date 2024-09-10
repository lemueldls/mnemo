import { invoke } from "@tauri-apps/api/core";
import type { Space } from ".";

export async function listSpaces() {
  const { data: spaces } = await useAsyncData("list_spaces", async () => {
    return Object.fromEntries(await invoke<[string, Space][]>("list_spaces"));
  });

  const item = await useStorageItem("spaces", []);
  item.value = spaces.value;

  return spaces;
}

export async function readSpaceFile(
  kind: NoteKind,
  spaceId: string,
  path: string
) {
  const file = await useStorageItem(`spaces/${spaceId}/${kind}/${path}`, "");

  return invoke<string>("read_file", { kind, spaceId, path });
}

export async function syncSpaceFile(
  kind: NoteKind,
  spaceId: string,
  path: string,
  text: string
) {
  const file = await useStorageItem(`spaces/${spaceId}/${kind}/${path}`, "");
  file.value = text;

  await invoke("sync_file", { kind, spaceId, path, text });
}
