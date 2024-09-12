import { type StorageValue } from "unstorage";

const itemRefs: { [key: string]: Ref<StorageValue> } = {};

export async function useStorageItem<T extends StorageValue>(
  key: string,
  initialValue: T
) {
  if (key in itemRefs) return itemRefs[key] as Ref<T>;

  const { $storage } = useNuxtApp();

  const localItem = useLocalStorage<T>(`app:user:${key}`, initialValue);
  const item = ref(localItem);
  itemRefs[key] = ref(item!);

  $storage.getItem<T>(key, { initialValue }).then((value) => {
    item.value = value;
  });

  watchDebounced(
    item,
    async (value) => {
      item.value = value;
      await updateStorageItem(key, value);
    },
    { debounce: 500, maxWait: 0, deep: true }
  );

  return item;
}

const updateStorageItem = useThrottleFn(async (key, value) => {
  const { $storage } = useNuxtApp();
  await $storage.setItem(key, value!);
}, 500);

export async function useStorageKeys(base: string) {
  const { $storage } = useNuxtApp();

  return await $storage.getKeys(base);
}
