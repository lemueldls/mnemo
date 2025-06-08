import { createStorage, type StorageValue } from "unstorage";
import indexedDbDriver from "unstorage/drivers/indexedb";

const localDb = createStorage({
  driver: indexedDbDriver({ base: "app:" }),
});

const itemRefs: { [key: string]: Promise<Ref<unknown>> } = {};

function asyncComputedRef<T>(
  key: MaybeRefOrGetter<string>,
  handler: (key: string) => Promise<Ref<T>>,
) {
  const keyValue = toValue(key);

  if (keyValue in itemRefs) return itemRefs[keyValue] as Promise<Ref<T>>;

  // eslint-disable-next-line no-async-promise-executor
  const item = new Promise<Ref<T>>(async (resolve) => {
    const data = ref<T>();

    const item = ref(await handler(keyValue));
    let stopSync = watchImmediate(
      item,
      (item) => {
        console.log({ key: keyValue, item });

        data.value = item;
      },
      { deep: true },
    );

    watch(
      toRef(key),
      (key) => {
        stopSync();

        asyncComputedRef(key, handler).then((item) => {
          stopSync = watchImmediate(item, (item) => {
            data.value = item;
          });
        });
      },
      { flush: "sync" },
    );

    resolve(
      computed({
        get: () => data.value!,
        set(value) {
          item.value = value;
        },
      }),
    );
  });

  itemRefs[keyValue] = item;

  return item;
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
