import { createStorage, type StorageValue } from "unstorage";
import indexedDbDriver from "unstorage/drivers/indexedb";

import type { WatchStopHandle } from "vue";

const localDb = createStorage({
  driver: indexedDbDriver({ base: "app:" }),
});

const itemRefs: { [key: string]: Promise<Ref<unknown>> } = {};

async function asyncComputedRef<T>(
  key: MaybeRefOrGetter<string>,
  handler: (key: string) => Promise<Ref<T>>,
) {
  const data = ref<T>();

  let stopSync: WatchStopHandle;
  let item: Ref<T>;

  await new Promise<void>((resolve) =>
    watchImmediate(
      toRef(key),
      async (key) => {
        stopSync?.();

        itemRefs[key] ||= handler(key);
        item = (await itemRefs[key]) as Ref<T>;

        stopSync = watchImmediate(item, (item) => {
          data.value = item;
          resolve();
        });
      },
      { flush: "sync" },
    ),
  );

  return computed({
    get: () => data.value!,
    set(value) {
      item.value = value;
    },
  });
}

export function useStorageItem<T extends StorageValue>(
  key: MaybeRefOrGetter<string>,
  initialValue: T,
) {
  // const { $sync } = useNuxtApp();

  return asyncComputedRef(toRef(key), async (key) => {
    const localItem = await localDb.getItem<T>(key);
    if (!localItem) {
      await updateLocalItem(key, initialValue);

      // await updateRemoteItem(key, initialValue);
    }

    // $sync.getMeta(key).then(async (syncMeta) => {
    //   const localMeta = await localDb.getMeta(key);
    //   if (!localMeta) throw createError("local meta not found")

    //   if (syncMeta.updatedAt > localMeta.updatedAt) {
    //     const value = await $sync.getItem(key);
    //     updateLocalItem(key, value)
    //   }
    // });

    const item = ref(localItem || initialValue);
    // const { ready, loggedIn } = useUserSession();

    watchDebounced(
      item,
      async (value) => {
        console.log({ key, value });

        await localDb.setItem(key, value);
        await localDb.setMeta(key, { updatedAt: Date.now() });

        // if (loggedIn.value) await updateRemoteItem(key, value);
      },
      { debounce: 500, deep: true },
    );

    // whenever(logicAnd(ready, loggedIn), async () => {
    //   item.value = await $sync.getItem<T>(key, { initialValue });
    // });

    return item;
  });
}

async function updateLocalItem<T extends StorageValue>(key: string, value: T) {
  await localDb.setItem(key, value);
  await localDb.setMeta(key, { updatedAt: Date.now() });
}

// const updateRemoteItem = useThrottleFn(async (key, value) => {
//   const { $sync } = useNuxtApp();
//   await $sync.setItem(key, value!);
// }, 500);

export async function useStorageKeys(base: string) {
  const { $sync } = useNuxtApp();

  return await $sync.getKeys(base);
}
