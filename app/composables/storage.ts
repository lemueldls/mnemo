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

const syncQueue = new Set<string>();

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

const useSync = createSharedComposable(() => {
  const { ready, loggedIn } = useUserSession();

  let updateItemCallback: (
    key: string,
    value: StorageValue,
    updatedAt: number,
  ) => void;

  whenever(
    logicAnd(ready, loggedIn),
    () => {
      const { send } = useWebSocket("/api/user-storage", {
        async onMessage(_ws, event) {
          const { key, value, updatedAt } = JSON.parse(
            typeof event.data === "string"
              ? event.data
              : await event.data.text(),
          ) as { key: string; value: StorageValue; updatedAt: number };

          const meta = await localDb.getMeta(key);

          if (!meta.updatedAt || updatedAt > (meta.updatedAt as number)) {
            await localDb.setItem(key, value);
            await localDb.setMeta(key, { updatedAt });

            const itemRef = await itemRefs[key];

            if (itemRef && itemRef.value !== value) {
              syncQueue.add(key);
              itemRef.value = value;
            }
          }
        },
      });

      updateItemCallback = (
        key: string,
        value: StorageValue,
        updatedAt: number,
      ) => {
        send(JSON.stringify({ key, value, updatedAt }));
      };
    },
    { immediate: true },
  );

  return {
    updateItem(key: string, value: StorageValue, updatedAt: number) {
      updateItemCallback?.(key, value, updatedAt);
    },
  };
});

export function useStorageItem<T extends StorageValue>(
  key: MaybeRefOrGetter<string>,
  initialValue: T,
) {
  return asyncComputedRef(toRef(key), async (key) => {
    const storageItem = await getStorageItem<T>(key, initialValue);
    const item = ref(storageItem);

    watchDebounced(
      item,
      async (value) => {
        if (!syncQueue.delete(key)) {
          const updatedAt = Date.now();

          await localDb.setItem(key, value);
          await localDb.setMeta(key, { updatedAt });

          useSync().updateItem(key, toRaw(value), updatedAt);
        }
      },
      { debounce: 500, deep: true },
    );

    return item as Ref<T>;
  });
}

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
