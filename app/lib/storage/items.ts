import { normalizeKey, type StorageValue } from "unstorage";

import { commit, useCrdt } from "./crdt";
import { localDb } from "./db";
import { useSharedAsyncData, type StorageRef } from "./refs";
import { useSync } from "./sync";
import { deepEqual } from "./utils";

import type { Container } from "loro-crdt";

export function useStorageItem<T extends StorageValue>(
  key: MaybeRefOrGetter<string>,
  initialValue: T,
) {
  return createStorageItem(key, initialValue);
}

export async function useStorageBytes(key: MaybeRefOrGetter<string>, initialValue?: Uint8Array) {
  const item = await createStorageItem(key, initialValue ? initialValue.toBase64() : "");

  return computed({
    get: () => Uint8Array.fromBase64(item.value),
    set(bytes) {
      item.value = bytes.toBase64();
    },
  });
}

export async function useStorageText<T extends string>(
  key: MaybeRefOrGetter<string>,
  initialValue?: T,
) {
  const keyRef = computed(() => normalizeKey(toValue(key)));

  const item = await createStorageItem(
    keyRef,
    initialValue ?? ("" as T),
    async (key, item, runNextSync) => {
      const doc = await useCrdt();
      const text = doc.getText(key);

      watchImmediate(item, (itemText) => {
        if (!runNextSync.value) return;

        text.update(itemText);
        void commit();
      });
    },
  );

  return extendRef(item, {});
}

export type MapRef<T extends Record<string, unknown>> = Awaited<
  ReturnType<typeof useStorageMap<T>>
>;
export async function useStorageMap<T extends Record<string, unknown>>(
  key: MaybeRefOrGetter<string>,
  initialValue?: T,
) {
  const keyRef = computed(() => normalizeKey(toValue(key)));

  const item = await createStorageItem(
    keyRef,
    initialValue ?? ({} as T),
    async (key, item, runNextSync) => {
      const doc = await useCrdt();
      const map = doc.getMap(key);

      watchImmediate(item, (itemMap) => {
        if (!runNextSync.value) return;

        for (const [key, value] of Object.entries(itemMap)) map.set(key, value);

        void commit();
      });

      return item;
    },
  );

  const doc = await useCrdt();
  const map = computedWithControl(item, () => doc.getMap(keyRef.value));

  return extendRef(item, {
    async set(key: string, value: Exclude<T[keyof T], Container>) {
      // item.value[key as keyof T] = value;
      map.value.set(key, value);
      await commit();
    },
    async delete(key: string) {
      // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
      delete item.value[key];
      map.value.delete(key);
      await commit();
    },
  });
}

export type ListRef<T extends unknown[]> = Awaited<ReturnType<typeof useStorageList<T>>>;
export async function useStorageList<T extends unknown[]>(
  key: MaybeRefOrGetter<string>,
  initialValue?: T,
) {
  const keyRef = computed(() => normalizeKey(toValue(key)));

  const item = await createStorageItem(
    keyRef,
    initialValue ?? ([] as unknown as T),
    async (key, item, runNextSync) => {
      const doc = await useCrdt();
      const list = doc.getList(key);

      watchImmediate(item, (itemList) => {
        if (!runNextSync.value) return;

        const syncList = list.getShallowValue();
        const maxLength = Math.max(itemList.length, syncList.length);

        for (let i = 0; i < maxLength; i++)
          if (i < itemList.length && i < syncList.length) {
            if (!deepEqual(itemList[i], syncList[i])) {
              list.delete(i, 1);
              list.insert(i, itemList[i]);
            }
          } else if (i < itemList.length) list.push(itemList[i]);
          else list.delete(i, 1);

        void commit();
      });
    },
  );

  const doc = await useCrdt();
  const list = computedWithControl(item, () => doc.getList(keyRef.value));

  return extendRef(item, {
    async push(value: Exclude<T[keyof T], Container>) {
      list.value.push(value);
      await commit();
    },
    async insert(position: number, value: Exclude<T[keyof T], Container>) {
      list.value.insert(position, value);
      await commit();
    },
    async delete(position: number, length: number) {
      list.value.delete(position, length);
      await commit();
    },
  });
}

export type SetRef<T extends Exclude<{ [key: string]: any }, Container>[]> = Awaited<
  ReturnType<typeof useStorageSet<T>>
>;
export async function useStorageSet<T extends Exclude<{ [key: string]: any }, Container>[]>(
  key: MaybeRefOrGetter<string>,
  setKey: keyof T[number],
  initialValue?: T,
) {
  const keyRef = computed(() => normalizeKey(toValue(key)));

  const item = await createStorageItem(
    keyRef,
    initialValue ?? ([] as unknown as T),
    async (key, item, runNextSync) => {
      const doc = await useCrdt();
      const list = doc.getMovableList(key);

      watchImmediate(item, (itemList) => {
        if (!runNextSync.value) return;

        const syncKeys = new Set<string>();

        for (let i = 0; i < list.length; i++) {
          const item = list.get(i) as Exclude<T[number], Container>;
          const key: string = item[setKey];

          syncKeys.add(key);

          let moveIndex;

          for (let j = i + 1; j < list.length; j++) {
            const otherItem = list.get(j) as Exclude<T[number], Container>;
            const otherKey: string = otherItem[setKey];

            const order = key.localeCompare(otherKey);

            if (order === 0) list.delete(j, 1);
            else if (order > 0) moveIndex = j;
          }

          if (moveIndex) list.move(i, moveIndex);
        }

        for (const item of itemList) {
          const key: string = item[setKey as keyof typeof item];

          if (syncKeys.has(key)) continue;

          let left = 0;
          let right = list.length;

          while (left < right) {
            const mid = Math.floor((left + right) / 2);
            const midItem = list.get(mid) as Exclude<T[number], Container>;
            const midKey: string = midItem[setKey];

            if (midKey.localeCompare(key) < 0) left = mid + 1;
            else right = mid;
          }

          list.insert(left, item);
        }

        void commit();
      });
    },
  );

  const doc = await useCrdt();
  const list = computedWithControl(item, () => doc.getMovableList(keyRef.value));

  return extendRef(item, {
    async push(value: Exclude<T[keyof T], Container>) {
      list.value.push(value);
      await commit();
    },
    async insert(position: number, value: Exclude<T[keyof T], Container>) {
      list.value.insert(position, value);
      await commit();
    },
    async delete(position: number, length: number) {
      list.value.delete(position, length);
      await commit();
    },
  });
}

function createStorageItem<T extends StorageValue>(
  key: MaybeRefOrGetter<string>,
  initialValue: T,
  itemHook?: (key: string, item: StorageRef<T>, runNextSync: Ref<boolean>) => void,
) {
  return useSharedAsyncData(key, async (key, scope) => {
    const storageItem = await getStorageItem<T>(key);
    const item = ref(storageItem || initialValue) as Ref<T>;

    const customRef = scope.run(() => {
      const runNextSync = ref(true);

      watchThrottled(
        item,
        async (value: T) => {
          const updatedAt = Date.now();

          await localDb.setItem(key, value);
          await localDb.setMeta(key, { updatedAt });

          if (runNextSync.value) useSync().updateItem(key, value, updatedAt);
          else runNextSync.value = true;
        },
        { throttle: 1000, deep: true },
      );

      const customRef = extendRef(item, {
        setLocal(value: T) {
          runNextSync.value = false;
          item.value = value;
        },
      });

      itemHook?.(key, customRef, runNextSync);

      return customRef;
    });

    return customRef!;
  });
}

export async function getStorageItem<T extends StorageValue>(key: string) {
  const localItem = await localDb.getItem<T>(key);

  const localMeta = localDb.getMeta(key) as Promise<{ updatedAt?: number } | undefined>;

  localMeta.then((meta) => {
    useSync().updateItem(key, localItem, meta?.updatedAt ?? 0);
  });

  return localItem;
}
