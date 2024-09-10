import { type StorageValue } from "unstorage";

const itemRefs: { [key: string]: Ref<StorageValue> } = {};

export async function useStorageItem<T extends StorageValue>(
  key: string,
  initialValue: T
) {
  if (key in itemRefs) {
    const item = itemRefs[key] as Ref<T>;
    useLocalStorage(key, initialValue).value = item.value;

    return item;
  }

  const { $storage } = useNuxtApp();
  const localItem = useLocalStorage<T>(`app:user:${key}`, initialValue);

  const { data: item } = await useAsyncData<T>(
    `storage:${key}`,
    async () => {
      const item = await $storage.getItem<T>(key, { initialValue });
      itemRefs[key] = ref(item!) as Ref<T>;
      localItem.value = item!;

      return item! as Awaited<T>;
    },
    { default: () => localItem }
  );

  watchDebounced(
    item,
    async (value) => {
      localItem.value = value as T;
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
