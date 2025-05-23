import { createStorage, type StorageValue } from "unstorage";
import indexedDbDriver from "unstorage/drivers/indexedb";

const localDb = createStorage({
  driver: indexedDbDriver({ base: "app:" }),
});

const itemRefs: { [key: string]: Ref<StorageValue> } = {};

export async function useStorageItem<T extends StorageValue>(
  key: string,
  initialValue: T,
) {
  if (key in itemRefs) return itemRefs[key] as Ref<T>;

  const localItem = await localDb.getItem<T>(key);
  if (!localItem) {
    await updateLocalItem(key, initialValue);

    // await updateStorageItem(key, initialValue);
  }

  // const { $sync } = useNuxtApp();

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
      // console.log({ key, value });

      await localDb.setItem(key, value);
      await localDb.setMeta(key, { updatedAt: Date.now() });

      // if (loggedIn.value) await updateStorageItem(key, value);
    },
    { debounce: 500, deep: true },
  );

  itemRefs[key] = item!;

  // whenever(logicAnd(ready, loggedIn), async () => {
  //   item.value = await $sync.getItem<T>(key, { initialValue });
  // });

  return item;
}

async function updateLocalItem<T extends StorageValue>(key: string, value: T) {
  await localDb.setItem(key, value);
  await localDb.setMeta(key, { updatedAt: Date.now() });
}

export async function useRefStorageItem<T extends StorageValue>(
  key: Ref<string>,
  initialValue: T,
) {
  const { error, data: item } = await useAsyncData(
    `app:${key.value}`,
    async () => await useStorageItem(key.value, initialValue),
    { watch: [key] },
  );

  watchImmediate(error, (error) => {
    if (error) throw createError(error);
  });

  return flattenRef(item) as Ref<T>;
}

export function flattenRef<T>(root: Ref<Ref<T>>) {
  const newRoot = ref();

  let triggerNewRoot = true;
  let triggerRoot = true;

  watch(newRoot, () => {
    if (triggerNewRoot) {
      triggerRoot = false;
      root.value.value = newRoot.value;
    } else triggerNewRoot = true;
  });

  watchImmediate([root, root.value], () => {
    if (triggerRoot) {
      triggerNewRoot = false;
      newRoot.value = root.value?.value;
    } else triggerRoot = true;
  });

  return newRoot as Ref<T>;
}

const updateStorageItem = useThrottleFn(async (key, value) => {
  const { $sync } = useNuxtApp();
  await $sync.setItem(key, value!);
}, 500);

export async function useStorageKeys(base: string) {
  const { $sync } = useNuxtApp();

  return await $sync.getKeys(base);
}
