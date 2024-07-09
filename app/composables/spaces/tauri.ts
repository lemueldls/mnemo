import {invoke} from "@tauri-apps/api/core"
import { Store } from "@tauri-apps/plugin-store";
import type { Space } from ".";

import type { MaterialSymbol } from "material-symbols";

const store = new Store("spaces.json");
const spaces = shallowRef<{ [key: string]: Space }>({});

if (import.meta.client) {
  await store.clear();

  await store.set("General Chemistry II", {
    order: 0,
    icon: "experiment",
    color: "#ef4444",
  });

  await store.set("Calculus II", {
    order: 1,
    icon: "function",
    color: "#06b6d4",
  });

  await store.set("US Politics and Government", {
    order: 2,
    icon: "gavel",
    color: "#10b981",
  });

  await store.set("Logical Reasoning", {
    order: 3,
    icon: "psychology",
    color: "#eab308",
  });

  await store.save();

  const entries = ref(await store.entries<Space>());

  store.onChange(async () => {
    entries.value = await store.entries<Space>();
  });

  entries.value.sort(([, a], [, b]) => a.order - b.order);

  spaces.value = Object.fromEntries(entries.value);
}

export function listSpaces() {
  return spaces;
}

export function readSpaceFile(space: string, path: string) {
  return invoke<string>("read_file", { space, path });
}

export async function syncSpaceFile(space: string, path: string, text: string) {
  await invoke("sync_file", { space, path, text });
}
