import { createStorage, normalizeKey, type StorageValue } from "unstorage";
import indexedDbDriver from "unstorage/drivers/indexedb";

import type { Container, Value } from "loro-crdt";

import type {
  DebuggerOptions,
  WatchStopHandle,
  WritableComputedOptions,
} from "vue";

const localDb = createStorage({
  driver: indexedDbDriver({ base: "app:" }),
});

type CustomRef<T> = Ref<T> & { setLocal(value: T): Promise<void> };

const itemRefs: { [key: string]: Promise<CustomRef<unknown>> | undefined } = {};
const itemRefCount: { [key: string]: number } = {};

declare global {
  interface Uint8Array {
    toBase64(): string;
  }

  interface Uint8ArrayConstructor {
    fromBase64(this: this, base64: string): Uint8Array;
  }
}

if (!Uint8Array.prototype.toBase64) {
  Uint8Array.prototype.toBase64 = function () {
    let binaryString = "";
    for (let i = 0; i < this.length; i++) {
      binaryString += String.fromCharCode(this[i]!);
    }

    return btoa(binaryString);
  };
}

if (!Uint8Array.fromBase64) {
  Uint8Array.fromBase64 = function (base64String) {
    // Decode the Base64 string to a string of binary data
    const binaryString = atob(base64String);
    // Create a Uint8Array from the binary string
    const len = binaryString.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes;
  };
}

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

  doc.subscribe(async (event) => {
    for (const { path, diff } of event.events) {
      // console.log("[CRDT]", path, diff);

      const key = normalizeKey(path[0] as string);

      switch (diff.type) {
        case "text": {
          const text = doc.getText(key).getShallowValue();

          // let cursor = 0;
          // for (const delta of diff.diff)
          //   if (delta.insert)
          //     text = text.slice(0, cursor) + delta.insert + text.slice(cursor);
          //   else if (delta.delete)
          //     text = text.slice(0, cursor) + text.slice(cursor + delta.delete);
          //   else if (delta.retain) cursor += delta.retain;

          await localDb.setItem(key, text);
          await localDb.setMeta(key, { updatedAt: Date.now() });

          const itemRef = await itemRefs[key];
          if (itemRef) await itemRef.setLocal(text);

          break;
        }

        case "map": {
          const item =
            (await localDb.getItem<Record<string, unknown>>(key)) ?? {};
          const itemClone = structuredClone(item);

          for (const [key, value] of Object.entries(diff.updated))
            if (value === null)
              // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
              delete item[key];
            else item[key] = value as Value;

          await localDb.setItem(key, item);
          await localDb.setMeta(key, { updatedAt: Date.now() });

          const itemRef = await itemRefs[key];
          if (itemRef) await itemRef.setLocal(item);

          break;
        }

        case "list": {
          const item = (await localDb.getItem<unknown[]>(key)) ?? [];

          for (const delta of diff.diff)
            if (delta.insert)
              for (let i = 0; i < delta.insert.length; i++)
                item[i] = delta.insert[i];
            else if (delta.delete) item.splice(delta.delete, item.length);

          await localDb.setItem(key, item);
          await localDb.setMeta(key, { updatedAt: Date.now() });

          const itemRef = await itemRefs[key];
          if (itemRef) await itemRef.setLocal(item);

          break;
        }
      }
    }

    console.log("sending local update by", event.by);
    const snapshot = doc.export({ mode: "snapshot" });
    await localDb.setItem("crdt", snapshot.toBase64());
    await send(snapshot);
  });

  const url = new URL("/api/crdt", useApiBaseUrl());
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

const useSync = createSharedComposable(() => {
  const url = new URL("/api/user-storage", useApiBaseUrl());
  url.protocol = url.protocol === "https:" ? "wss:" : "ws:";

  const { open, close, send } = useApiWebSocket(url, {
    immediate: false,
    async onMessage(_ws, event) {
      const text = await event.text();
      const { key, value, updatedAt } = JSON.parse(text) as {
        key: string;
        value: StorageValue;
        updatedAt: number;
      };

      const meta = await localDb.getMeta(key);

      if (!meta.updatedAt || updatedAt > (meta.updatedAt as number)) {
        await localDb.setItem(key, value);
        await localDb.setMeta(key, { updatedAt });

        const itemRef = await itemRefs[key];
        if (itemRef) await itemRef.setLocal(value);
      }
    },
  });

  const { loggedIn } = useAuth();
  watchImmediate([loggedIn, useOnline()], ([loggedIn, isOnline]) => {
    if (loggedIn && isOnline) open();
    else close();
  });

  return {
    async updateItem(key: string, value: StorageValue, updatedAt: number) {
      await send(JSON.stringify({ key, value, updatedAt }));
    },
  };
});

function shallowComputed<T, S = T>(
  options: WritableComputedOptions<T, S>,
  debugOptions?: DebuggerOptions,
): WritableComputedRef<T, S> {
  const root = computed(options, debugOptions);
  Object.defineProperty(root, "__v_isShallow", {
    configurable: true,
    enumerable: false,
    value: true,
  });

  return root;
}

async function asyncComputedRef<T>(
  key: MaybeRefOrGetter<string>,
  handler: (key: string) => Promise<CustomRef<T>>,
) {
  let item: Ref<T>;
  const data = shallowRef<T>();

  const root = shallowComputed({
    get: () => data.value!,
    set(value) {
      item.value = value;
    },
  });

  const keyRef = computed(() => normalizeKey(toValue(key)));

  let stopSync: WatchStopHandle;
  await new Promise<void>((resolve) =>
    watchImmediate(keyRef, async (key) => {
      itemRefCount[key] ??= 0;
      itemRefCount[key]++;

      const scope = effectScope();

      onWatcherCleanup(() => {
        stopSync?.();

        if (!itemRefCount[key] || itemRefCount[key] <= 1) {
          // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
          delete itemRefs[key];
          // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
          delete itemRefCount[key];
        } else itemRefCount[key]--;
      });

      itemRefs[key] ??= handler(key);
      item = (await itemRefs[key]) as CustomRef<T>;

      scope.run(() => {
        stopSync = watchImmediate(item, (item) => {
          data.value = item;
        });

        resolve();
        triggerRef(root);
      });
    }),
  );

  return root;
}

export function useStorageItem<T extends StorageValue>(
  key: MaybeRefOrGetter<string>,
  initialValue: T,
) {
  return asyncComputedRef(key, async (key) => {
    const scope = effectScope();

    const storageItem = await getStorageItem<T>(key, initialValue);
    const item = ref(storageItem) as Ref<T>;

    const customRef = scope.run(() => {
      let runNextSync = true;

      watchThrottled(
        item,
        async (value: T) => {
          const updatedAt = Date.now();

          await localDb.setItem(key, value);
          await localDb.setMeta(key, { updatedAt });

          if (runNextSync) useSync().updateItem(key, value, updatedAt);
          else runNextSync = true;
        },
        { throttle: 1000, deep: true },
      );

      return extendRef(item, {
        async setLocal(value: T) {
          runNextSync = false;
          item.value = value;
        },
      });
    });

    return customRef!;
  });
}

export async function useStorageText<T extends string>(
  key: MaybeRefOrGetter<string>,
  initialValue?: T,
) {
  const keyRef = computed(() => normalizeKey(toValue(key)));
  const item = await useStorageItem(keyRef, initialValue ?? ("" as T));

  const doc = await useCrdt();
  const text = computedWithControl(item, () => doc.getText(keyRef.value));

  watchImmediate(item, (itemText) => {
    text.value.update(itemText);
    commit();
  });

  return extendRef(item, {});
}

export type MapRef<T extends Record<string, unknown>> = Awaited<
  ReturnType<typeof useStorageMap<T>>
>;
export async function useStorageMap<T extends Record<string, unknown>>(
  key: MaybeRefOrGetter<string>,
  initialValue?: T,
) {
  const keyRef = computed(() => normalizeKey(toValue(key)));
  const item = await useStorageItem<T>(keyRef, initialValue ?? ({} as T));

  const doc = await useCrdt();
  const map = computedWithControl(item, () => doc.getMap(keyRef.value));

  watchImmediate(item, (itemMap) => {
    for (const [key, value] of Object.entries(itemMap))
      map.value.set(key, value);
    commit();
  });

  return extendRef(item, {
    set(key: string, value: Exclude<T[keyof T], Container>) {
      // item.value[key as keyof T] = value;
      map.value.set(key, value);
      commit();
    },
    delete(key: string) {
      // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
      delete item.value[key];
      map.value.delete(key);
      commit();
    },
  });
}

export async function useStorageList<T extends unknown[]>(
  key: MaybeRefOrGetter<string>,
  initialValue?: T,
) {
  const keyRef = computed(() => normalizeKey(toValue(key)));
  const item = await useStorageItem<T>(
    keyRef,
    initialValue ?? ([] as unknown as T),
  );

  const doc = await useCrdt();
  const list = computedWithControl(item, () => doc.getList(keyRef.value));

  watchImmediate(item, (itemList) => {
    const syncList = list.value.getShallowValue();
    const maxLength = Math.max(itemList.length, syncList.length);
    for (let i = 0; i < maxLength; i++)
      if (i < itemList.length && i < syncList.length) {
        if (!deepEqual(itemList[i], syncList[i])) {
          list.value.delete(i, 1);
          list.value.insert(i, itemList[i]);
        }
      } else if (i < itemList.length) list.value.push(itemList[i]);
      else list.value.delete(i, 1);
    commit();
  });

  return extendRef(item, {
    push(value: Exclude<T[keyof T], Container>) {
      list.value.push(value);
      commit();
    },
    insert(position: number, value: Exclude<T[keyof T], Container>) {
      list.value.insert(position, value);
      commit();
    },
    delete(position: number, length: number) {
      list.value.delete(position, length);
      commit();
    },
  });
}

export type MovableListRef<T extends unknown[]> = Awaited<
  ReturnType<typeof useStorageMovableList<T>>
>;
export async function useStorageMovableList<T extends unknown[]>(
  key: MaybeRefOrGetter<string>,
  initialValue?: T,
) {
  const keyRef = computed(() => normalizeKey(toValue(key)));
  const item = await useStorageItem<T>(
    keyRef,
    initialValue ?? ([] as unknown as T),
  );

  const doc = await useCrdt();
  const list = computedWithControl(item, () =>
    doc.getMovableList(keyRef.value),
  );

  watchImmediate(item, (itemList) => {
    const syncList = list.value.getShallowValue();
    const maxLength = Math.max(itemList.length, syncList.length);
    for (let i = 0; i < maxLength; i++)
      if (i < itemList.length && i < syncList.length) {
        if (!deepEqual(itemList[i], syncList[i]))
          list.value.set(i, itemList[i]);
      } else if (i < itemList.length) list.value.push(itemList[i]);
      else list.value.delete(i, 1);
    commit();
  });

  return extendRef(item, {
    push(value: Exclude<T[keyof T], Container>) {
      list.value.push(value);
      commit();
    },
    insert(position: number, value: Exclude<T[keyof T], Container>) {
      list.value.insert(position, value);
      commit();
    },
    set(position: number, value: Exclude<T[keyof T], Container>) {
      list.value.set(position, value);
      commit();
    },
    delete(position: number, length: number) {
      list.value.delete(position, length);
      commit();
    },
  });
}

const commit = useThrottleFn(
  async () => {
    const doc = await useCrdt();
    doc.commit();
  },
  1000,
  true,
  true,
);

export async function getStorageItem<T extends StorageValue>(
  key: string,
  initialValue: T,
) {
  const localItem = await localDb.getItem<T>(key);

  if (!localItem) {
    await localDb.setItem(key, initialValue);
    await localDb.setMeta(key, { updatedAt: 0 });
  }

  const value = localItem ?? initialValue;

  const localMeta = localDb.getMeta(key) as Promise<
    { updatedAt?: number } | undefined
  >;

  localMeta.then((meta) => {
    useSync().updateItem(key, value, meta?.updatedAt ?? 0);
  });

  return value;
}

function deepEqual(a: unknown, b: unknown): boolean {
  if (a === b) return true;

  if (a === null || b === null) return false;
  if (typeof a !== typeof b) return false;

  if (Array.isArray(a) && Array.isArray(b)) {
    if (a.length !== b.length) return false;

    for (let i = 0; i < a.length; i++) {
      if (!deepEqual(a[i], b[i])) return false;
    }

    return true;
  }

  if (typeof a === "object" && typeof b === "object") {
    const keysA = Object.keys(a as Record<string, unknown>);
    const keysB = Object.keys(b as Record<string, unknown>);

    if (keysA.length !== keysB.length) return false;

    for (const key of keysA) {
      if (
        !deepEqual(
          (a as Record<string, unknown>)[key],
          (b as Record<string, unknown>)[key],
        )
      )
        return false;
    }

    return true;
  }

  return false;
}
