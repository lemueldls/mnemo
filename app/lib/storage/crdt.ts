import { match } from "ts-pattern";
import { normalizeKey } from "unstorage";

import { localDb } from "./db";
import { itemsRefs } from "./refs";

export const useLoro = createSharedComposable(() => import("loro-crdt"));

export const useCrdt = createSharedComposable(async () => {
  const { LoroDoc } = await useLoro();

  let doc;
  const bytes = await localDb.getItem<string>("crdt");
  if (bytes) {
    try {
      doc = LoroDoc.fromSnapshot(Uint8Array.fromBase64(bytes));
    } catch (error) {
      console.error("Failed to load CRDT from local storage:", error);
      await localDb.removeItem("crdt");
      doc = new LoroDoc();
    }
  } else doc = new LoroDoc();

  const syncSnapshot = useThrottleFn(
    async () => {
      const snapshot = doc.export({ mode: "snapshot" });
      await localDb.setItem("crdt", snapshot.toBase64());
    },
    1000,
    true,
    true,
  );

  doc.subscribeLocalUpdates((updates) => send(updates));

  doc.subscribe(async (event) => {
    for (const { path, diff } of event.events) {
      const key = normalizeKey(path[0] as string);

      const item = match(diff.type)
        .with("text", () => doc.getText(key))
        .with("map", () => doc.getMap(key))
        .with("list", () => doc.getList(key))
        .with("counter", () => doc.getCounter(key))
        .with("tree", () => doc.getTree(key))
        .exhaustive()
        .getShallowValue();

      await localDb.setItem(key, item);
      await localDb.setMeta(key, { updatedAt: Date.now() });

      const itemRef = await itemsRefs[key];
      if (itemRef) itemRef.setLocal(item);
    }

    syncSnapshot();
  });

  const token = useApiToken().value;
  const url = new URL(`/api/crdt?token=${token}`, useApiBaseUrl());
  url.protocol = url.protocol === "https:" ? "wss:" : "ws:";

  const { open, close, send } = useApiWebSocket(url, {
    immediate: false,
    async onOpen() {
      const snapshot = doc.export({ mode: "snapshot" });
      await send(snapshot);
    },
    async onMessage(_ws, event) {
      const bytes = await event.bytes();
      doc.import(bytes);
    },
  });

  const { loggedIn } = useAuth();
  watchImmediate([loggedIn, useOnline()], ([loggedIn, isOnline]) => {
    if (loggedIn && isOnline) open();
    else close();
  });

  return doc;
});

export const useCrdtUndoManager = createSharedComposable(async () => {
  const { UndoManager } = await useLoro();
  const doc = await useCrdt();

  const undoManager = new UndoManager(doc, {
    maxUndoSteps: 100,
    mergeInterval: 1000,
  });

  return undoManager;
});

export const commit = useThrottleFn(
  async () => {
    const doc = await useCrdt();
    doc.commit();
  },
  1000,
  true,
  true,
);
