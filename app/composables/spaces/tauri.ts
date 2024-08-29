import { invoke } from "@tauri-apps/api/core";
import { Store } from "@tauri-apps/plugin-store";
import type { Space } from ".";

import type { MaterialSymbol } from "material-symbols";

const store = new Store("spaces.json");
const spaces = shallowRef<{ [key: string]: Space }>({});

if (import.meta.client) {
  // await store.clear();
  // await store.set("Two-Dimensional Design", {
  //   order: 0,
  //   icon: "draw",
  //   color: "#ef4444",
  // });
  // await store.set("Calculus II", {
  //   order: 1,
  //   icon: "function",
  //   color: "#06b6d4",
  // });
  // await store.set("Music Theory Fundamentals", {
  //   order: 2,
  //   icon: "music_note",
  //   color: "#a855f7",
  // });
  // await store.set("Introduction to Logic", {
  //   order: 3,
  //   icon: "psychology",
  //   color: "#eab308",
  // });
  // await invoke("create_space", {
  //   name: "Two-Dimensional Design",
  //   icon: "draw",
  //   color: "#ef4444",
  //   order: 0,
  // });
  // await invoke("create_space", {
  //   name: "Calculus II",
  //   icon: "function",
  //   color: "#06b6d4",
  //   order: 1,
  // });
  // await invoke("create_space", {
  //   name: "Music Theory Fundamentals",
  //   icon: "music_note",
  //   color: "#a855f7",
  //   order: 2,
  // });
  // await invoke("create_space", {
  //   name: "Introduction to Logic",
  //   icon: "psychology",
  //   color: "#eab308",
  //   order: 3,
  // });
  // const entries = ref(await store.entries<Space>());
  // store.onChange(async () => {
  //   entries.value = await store.entries<Space>();
  // });
  // entries.value.sort(([, a], [, b]) => a.order - b.order);
  // spaces.value = Object.fromEntries(entries.value);
}

export async function listSpaces() {
  const { data: spaces } = await useAsyncData("list_spaces", async () => {
    return await invoke<[string, Space][]>("list_spaces");
  });

  return spaces;
}

export async function readSpaceFile(
  kind: NoteKind,
  spaceId: string,
  path: string
) {
  return invoke<string>("read_file", { kind, spaceId, path });
}

export async function syncSpaceFile(
  kind: NoteKind,
  spaceId: string,
  path: string,
  text: string
) {
  await invoke("sync_file", { kind, spaceId, path, text });
}
