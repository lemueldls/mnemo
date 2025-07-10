import type { Container } from "loro-crdt";
import { createStorage, type StorageValue } from "unstorage";
import indexedDbDriver from "unstorage/drivers/indexedb";

const localDb = createStorage({
  driver: indexedDbDriver({ base: "app:" }),
});

const itemRefs: { [key: string]: Ref<unknown> | undefined } = {};

export const useCrdt = createSharedComposable(async () => {
  const { LoroDoc } = await import("loro-crdt");

  const doc = new LoroDoc();

  // const undoManager = new UndoManager(doc, {
  //   maxUndoSteps: 100,
  //   mergeInterval: 1000,
  // });

  const bytes = await localDb.getItemRaw("crdt");
  if (bytes) doc.import(bytes);

  const runtimeConfig = useRuntimeConfig();
  const { apiBaseUrl } = runtimeConfig.public;

  const endpoint = "/api/crdt";
  const url = apiBaseUrl ? new URL(endpoint, apiBaseUrl) : endpoint;

  const { open, send } = useWebSocket(url, {
    immediate: false,
    async onMessage(_ws, event) {
      const bytes = await event.data.bytes();
      doc.import(bytes);

      await localDb.setItemRaw("crdt", bytes);
    },
  });

  const { loggedIn } = useAuth();
  whenever(loggedIn, open, { immediate: true });

  doc.subscribeLocalUpdates((bytes) => {
    send(bytes.buffer as ArrayBuffer);
  });

  doc.subscribe(async (event) => {
    if (event.by === "import") {
      for (const { path, diff } of event.events) {
        console.log({ path, diff });

        const [key] = path as [string];

        switch (diff.type) {
          case "map": {
            const localItem =
              await localDb.getItem<Record<string, unknown>>(key);
            const item = localItem || {};

            for (const [key, value] of Object.entries(diff.updated))
              item[key] = value;

            const itemRef = itemRefs[key];

            await localDb.setItem(key, item);
            await localDb.setMeta(key, { updatedAt: Date.now() });

            if (itemRef && itemRef.value !== item) {
              itemRef.value = item;
            }

            break;
          }
        }
      }
    }
  });

  return doc;
});

const useSync = createSharedComposable(() => {
  const runtimeConfig = useRuntimeConfig();
  const { apiBaseUrl } = runtimeConfig.public;

  const endpoint = "/api/user-storage";
  const url = apiBaseUrl ? new URL(endpoint, apiBaseUrl) : endpoint;

  const { open, send } = useWebSocket(url, {
    immediate: false,
    async onMessage(_ws, event) {
      const { key, value, updatedAt } = JSON.parse(
        typeof event.data === "string" ? event.data : await event.data.text(),
      ) as { key: string; value: StorageValue; updatedAt: number };

      const meta = await localDb.getMeta(key);

      if (!meta.updatedAt || updatedAt > (meta.updatedAt as number)) {
        const itemRef = itemRefs[key];

        await localDb.setItem(key, value);
        await localDb.setMeta(key, { updatedAt });

        if (itemRef && itemRef.value !== value) {
          itemRef.value = value;
        }
      }
    },
  });

  const { loggedIn } = useAuth();
  whenever(loggedIn, open, { immediate: true });

  return {
    updateItem(key: string, value: StorageValue, updatedAt: number) {
      send(JSON.stringify({ key, value, updatedAt }));
    },
  };
});

export async function useStorageItem<T extends StorageValue>(
  key: MaybeRefOrGetter<string>,
  initialValue: T,
) {
  const keyRef = toRef(key);

  const { error, data, pending } = await useAsyncData(key, async () => {
    const key = keyRef.value;

    const storageItem = await getStorageItem<T>(key, initialValue);
    const item = ref(storageItem);

    itemRefs[key] = item;

    watchThrottled(
      item,
      async (value) => {
        // if (!syncQueue.delete(key)) {
        const updatedAt = Date.now();

        await localDb.setItem(key, value);
        await localDb.setMeta(key, { updatedAt });

        useSync().updateItem(key, value, updatedAt);
        // }
      },
      { throttle: 1000, deep: true },
    );

    return item as Ref<T>;
  });

  watchImmediate(error, (error) => {
    if (error) throw createError(error);
  });

  const root = computed<T>({
    get: (oldValue) => (pending.value ? oldValue! : data.value!.value),
    set: (value) => {
      data.value!.value = value;
    },
  });

  Object.defineProperty(root, "__v_isShallow", {
    configurable: true,
    enumerable: false,
    value: true,
  });

  whenever(logicNot(pending), () => {
    triggerRef(root);
  });

  return root;
}

export async function useStorageText(key: string, initialValue?: string) {
  const item = await useStorageItem(key, initialValue || "");

  const doc = await useCrdt();
  const text = doc.getText(key);

  const computedRef = computed({
    get: () => item.value,
    set: (newText) => {
      item.value = newText;
      text.update(newText);

      commit();
    },
  });

  return extendRef(computedRef, {});
}

export async function useStorageMap<T extends Record<string, unknown>>(
  key: string,
  initialValue?: T,
) {
  const item = await useStorageItem<T>(key, initialValue || ({} as T));

  const doc = await useCrdt();
  const map = doc.getMap(key);

  const computedRef = computed({
    get: () => item.value,
    set: (newMap) => {
      item.value = newMap;
      for (const [key, value] of Object.entries(newMap)) map.set(key, value);

      commit();
    },
  });

  return extendRef(computedRef, {
    set(key: string, value: Exclude<T[keyof T], Container>) {
      item.value[key as keyof T] = value;
      map.set(key, value);

      commit();
    },
    delete(key: string) {
      // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
      delete item.value[key as keyof T];
      map.delete(key);

      commit();
    },
  });
}

const commit = useThrottleFn(
  async () => {
    const doc = await useCrdt();
    const bytes = doc.export({ mode: "update" });
    await localDb.setItemRaw("crdt", bytes);
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
    await localDb.setMeta(key, { updatedAt: Date.now() });
  }

  const value = localItem || initialValue;

  const localMeta = localDb.getMeta(key) as Promise<
    { updatedAt?: number } | undefined
  >;

  localMeta.then((meta) => {
    useSync().updateItem(key, value, meta?.updatedAt || Date.now());
  });

  return value;
}

// const updateRemoteItem = useThrottleFn(async (key, value) => {
//   const { $sync } = useNuxtApp();
//   await $sync.setItem(key, value!);
// }, 500);

// export async function useStorageKeys(base: string) {
//   const { $sync } = useNuxtApp();

//   return await $sync.getKeys(base);
// }
