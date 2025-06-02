import { createStorage, defineDriver } from "unstorage";

const myStorageDriver = defineDriver((options) => {
  return {
    name: "tauri-driver",
    options,
    async hasItem(key, _opts) {},
    async getItem(key, _opts) {},
    async setItem(key, value, _opts) {},
    async removeItem(key, _opts) {},
    async getKeys(base, _opts) {},
    async clear(base, _opts) {},
    async dispose() {},
    async watch(callback) {},
  };
});

const storage = createStorage({
  driver: myStorageDriver(),
});
