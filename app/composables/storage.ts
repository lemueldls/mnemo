import type { StorageValue } from "unstorage";

const itemRefs: { [key: string]: Ref<StorageValue> } = {};

export async function useStorageItem(key: string, initialValue: StorageValue) {
  const { $storage } = useNuxtApp();

  if (key in itemRefs) return itemRefs[key]!;

  const item = ref(await $storage.getItem(key));
  itemRefs[key] = item;

  watchThrottled(
    item,
    async (value) => {
      await $storage.setItem(key, value);
    },
    { throttle: 5000 }
  );

  if (item.value === undefined && initialValue !== undefined)
    item.value = initialValue;

  return item;
}
