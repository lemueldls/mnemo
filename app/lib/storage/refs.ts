import { normalizeKey } from "unstorage";

import type { DebuggerOptions, EffectScope, WatchStopHandle, WritableComputedOptions } from "vue";

export type StorageRef<T> = Ref<T> & { setLocal(value: T): void };

export const itemsRefs: Record<string, Promise<StorageRef<unknown>> | undefined> = {};
export const itemRefsCount: Record<string, number> = {};

export async function useSharedAsyncData<T>(
  key: MaybeRefOrGetter<string>,
  handler: (key: string, scope: EffectScope) => Promise<StorageRef<T>>,
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
      itemRefsCount[key] ??= 0;
      itemRefsCount[key]++;

      const scope = effectScope(true);

      onWatcherCleanup(() => {
        stopSync?.();

        if (!itemRefsCount[key] || itemRefsCount[key] <= 1) {
          scope.stop();

          // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
          delete itemsRefs[key];
          // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
          delete itemRefsCount[key];
        } else itemRefsCount[key]--;
      });

      itemsRefs[key] ??= handler(key, scope);
      item = (await itemsRefs[key]) as StorageRef<T>;

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
