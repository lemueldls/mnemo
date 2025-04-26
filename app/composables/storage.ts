import { createId } from "@paralleldrive/cuid2";
import { createStorage, snapshot, type StorageValue } from "unstorage";
import indexedDbDriver from "unstorage/drivers/indexedb";

const localDb = createStorage({
  driver: indexedDbDriver({ base: "app:" }),
});

// const i = "1"; // mbhdsscpbc32qwor9a692fpe - nz8qqnr8i1d6n2ayy1oms8lk
// const id = createId();
// const id = "nz5x3j9umirtfbxemqwumoe2";
// const keys = await localDb.getKeys(`app:spaces:${i}`);
// console.log({ keys });
// console.log({ keys: await localDb.getKeys(`app::spaces`) });

// const spaces = await localDb.getItem("spaces.json");
// console.log({ spaces });
// spaces[id] = spaces[i];
// delete spaces[i];
// await localDb.setItem("spaces.json", spaces);
// console.log(spaces, Object.entries(spaces));

// const schedule = await localDb.getItem("schedule.json", []);
// console.log({ schedule });

// for (const weekday of schedule) {
//   for (const item of weekday) {
//     // console.log({ item });
//     if (item.spaceId === "0") item.spaceId = "j77m2xuea1l39ewaxvl4sfl9";
//     if (item.spaceId === "1") item.spaceId = "nz5x3j9umirtfbxemqwumoe2";
//   }
// }

// await localDb.setItem("schedule.json", schedule);

// await localDb.setItem(
//   "spaces.json",
//   Object.fromEntries(spaces.map((space, i) => [i, space])),
// );

// for await (const key of keys) {
//   const k = key.slice(4);

//   if (key.includes("undefined") || key.includes("[object Object]")) {
//     await localDb.removeItem(k, { removeMeta: true });
//   }

//   // const content = await localDb.getItem(k);

//   // console.log({ k, content });

//   // await localDb.setItem(k.replace(i.toString(), id), content);
//   // await localDb.removeItem(k, { removeMeta: true });
// }

// for await (const key of await localDb.getKeys()) {
//   const k = key.slice(4);

//   if (key.includes("undefined") || key.includes("[object Object]")) {
//     await localDb.removeItem(k, { removeMeta: true });
//   }

//   console.log({ k });
// }

// for await (const [key, value] of Object.entries(await snapshot(localDb, "/"))) {
//   // console.log({ key, value });
//   if (key.includes("sticky")) console.log(await localDb.getItem(key.slice(4)));
// }

// console.log({ snapshot: await snapshot(localDb) });

// const { status, data, send, open, close } = useWebSocket("/api/user-storage", { immediate: false });

const itemRefs: { [key: string]: Ref<StorageValue> } = {};

export async function useStorageItem<T extends StorageValue>(
  key: string,
  initialValue: T
) {
  // console.log({ key });
  if (key in itemRefs) return itemRefs[key] as Ref<T>;

  const localItem = await localDb.getItem<T>(key);
  // console.log({ localItem });
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
    { debounce: 500, deep: true }
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
  initialValue: T
) {
  const { error, data: item } = await useAsyncData(
    `app:${key.value}`,
    async () => await useStorageItem(key.value, initialValue),
    { watch: [key] }
  );

  watchImmediate(error, (error) => {
    if (error) throw createError(error);
  });

  return flattenRef(item) as Ref<T>;
}

export function flattenRef<T>(root: Ref<Ref<T>>) {
  const newRoot = ref();

  // watch(root, () => console.log("trigger 1"));
  // watch(root.value, () => console.log("trigger 2"));
  // watch(newRoot, () => console.log("trigger 3"));

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
      newRoot.value = root.value.value;
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
