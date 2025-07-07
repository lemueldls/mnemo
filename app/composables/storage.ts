import { createStorage, type StorageValue } from "unstorage";
import indexedDbDriver from "unstorage/drivers/indexedb";

import {
  onWatcherCleanup,
  type DebuggerOptions,
  type WatchStopHandle,
  type WritableComputedOptions,
} from "vue";

const localDb = createStorage({
  driver: indexedDbDriver({ base: "app:" }),
});

const itemRefs: { [key: string]: Promise<Ref<unknown>> | undefined } = {};
const itemRefCount: { [key: string]: number } = {};

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
  handler: (key: string) => Promise<Ref<T>>,
) {
  let item: Ref<T>;
  const data = shallowRef<T>();

  const root = shallowComputed({
    get: () => data.value!,
    set(value) {
      item.value = value;
    },
  });

  let stopSync: WatchStopHandle;
  await new Promise<void>((resolve) =>
    watchImmediate(toRef(key), async (key) => {
      itemRefCount[key] ??= 0;
      itemRefCount[key]++;

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
      item = (await itemRefs[key]) as Ref<T>;

      tryOnScopeDispose(async () => {
        stopSync?.();

        itemRefs[key] = undefined;
        item = await handler(key);

        const firstItem = itemRefs[key] as unknown as Promise<Ref<T>>;
        if (firstItem) item = await firstItem;
        else itemRefs[key] = Promise.resolve(item);

        stopSync = watchImmediate(item, (item) => {
          data.value = item;
        });
      });

      stopSync = watchImmediate(item, (item) => {
        data.value = item;
      });

      resolve();
      triggerRef(root);
    }),
  );

  return root;
}

export function useStorageItem<T extends StorageValue>(
  key: MaybeRefOrGetter<string>,
  initialValue: T,
) {
  // const { $sync } = useNuxtApp();

  return asyncComputedRef(toRef(key), async (key) => {
    const storageItem = await getStorageItem<T>(key, initialValue);

    const item = ref(storageItem);
    // const { ready, loggedIn } = useUserSession();

    watchDebounced(
      item,
      async (value) => {
        await localDb.setItem(key, value);
        await localDb.setMeta(key, { updatedAt: Date.now() });

        // if (loggedIn.value) await updateRemoteItem(key, value);
      },
      { debounce: 500, deep: true },
    );

    // whenever(logicAnd(ready, loggedIn), async () => {
    //   item.value = await $sync.getItem<T>(key, { initialValue });
    // });

    return item as Ref<T>;
  });
}

export async function getStorageItem<T extends StorageValue>(
  key: string,
  initialValue: T,
) {
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

  return localItem || initialValue;
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
