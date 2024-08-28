// import { Loro } from "loro-crdt";
// import type { LoroText } from "loro-wasm";

// import {
//   ref as storageRef,
//   getDownloadURL,
//   getBlob,
//   getBytes,
//   uploadBytes,
// } from "firebase/storage";
// import { ref as dbRef, set } from "firebase/database";

// import { createClient } from "@supabase/supabase-js";
// import { schema } from "./generated/client";

// // Initiate your Electric database
// const conn = await ElectricDatabase.init("myApp.db");
// const electric = await electrify(conn, schema, config);
// const token = data.session.access_token;
// await electric.connect(token);

import type { Space } from ".";

const store = new Map();
const spaces = shallowRef<{ [key: string]: Space }>({});

// if (import.meta.client) {
//   await store.clear();

//   await store.set("General Chemistry II", {
//     order: 0,
//     icon: "experiment",
//     color: "#ef4444",
//   });

//   await store.set("Calculus II", {
//     order: 1,
//     icon: "function",
//     color: "#06b6d4",
//   });

//   await store.set("US Politics and Government", {
//     order: 2,
//     icon: "gavel",
//     color: "#10b981",
//   });

//   await store.set("Logical Reasoning", {
//     order: 3,
//     icon: "psychology",
//     color: "#eab308",
//   });

//   const entries = ref([...store.entries<Space>()]);

//   entries.value.sort(([, a], [, b]) => a.order - b.order);

//   spaces.value = Object.fromEntries(entries.value);
// }

export function listSpaces() {
  console.log(spaces.value);

  return spaces;
}

let activeFile;

export async function readSpaceFile(
  kind: NoteKind,
  name: string,
  path: string
) {
  return "";
}

export async function syncSpaceFile(path: string, text: string) {}
