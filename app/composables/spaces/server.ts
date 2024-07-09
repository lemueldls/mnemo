import { Loro } from "loro-crdt";
import type { LoroText } from "loro-wasm";

// import {
//   ref as storageRef,
//   getDownloadURL,
//   getBlob,
//   getBytes,
//   uploadBytes,
// } from "firebase/storage";
// import { ref as dbRef, set } from "firebase/database";

import { createClient } from "@supabase/supabase-js";
import { ElectricDatabase, electrify } from "electric-sql/wa-sqlite";
// import { schema } from "./generated/client";

// // Initiate your Electric database
// const conn = await ElectricDatabase.init("myApp.db");
// const electric = await electrify(conn, schema, config);
// const token = data.session.access_token;
// await electric.connect(token);

import type { Space } from ".";

const store = new Map();
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

  const entries = ref([...store.entries<Space>()]);

  entries.value.sort(([, a], [, b]) => a.order - b.order);

  spaces.value = Object.fromEntries(entries.value);
}

export function listSpaces() {
  console.log(spaces.value);

  return spaces;
}

let activeFile;

export async function readSpaceFile(space: string, path: string) {
  const storage = useFirebaseStorage();
  const db = useDatabase();
  const user = useCurrentUser();

  // watchEffect(() => {
  //   console.log({ user: user.value });
  // });

  activeFile = dbRef(db, "spaces", user.value.uid, space, path);

  const doc = new Loro();
  const text: LoroText = doc.getText("text");
  text.insert(0, "Hello world!");

  // activeFile.value = doc.exportSnapshot();
  set(activeFile, doc.exportSnapshot());
  // console.log(activeFile.value);
  // console.log(doc.toJson()); // { "text": "Hello world!" }

  // console.log("LMFAO,", await getBytes(activeFile));
  return "";
}

export async function syncSpaceFile(path: string, text: string) {
  const storage = useFirebaseStorage();
  const user = useCurrentUser();

  return;

  //   .then(async (url) => {
  //     const content = await $fetch(url);

  //     console.log({ content });
  //   })
  //   .catch(async () => {
  //     console.log({ file });

  //     await uploadBytes(file, new Blob());
  //   });
}
