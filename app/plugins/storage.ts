import { createStorage, defineDriver, type StorageValue } from "unstorage";

export default defineNuxtPlugin({
  name: "mnemo:storage",
  dependsOn: ["mnemo:api"],
  setup(nuxtApp) {
    const driver = defineDriver((options) => {
      const { $api } = nuxtApp;

      return {
        options,
        name: "mnemo-driver",
        // async hasItem(key, _opts) {},
        async getItem(key, opts: { initialValue: string }) {
          return await $api("/api/user-storage/get-item", {
            method: "post",
            body: { key, initialValue: opts.initialValue },
          });
        },
        async setItem(key, value, _opts) {
          await $api("/api/user-storage/set-item", {
            method: "post",
            body: { key, value },
          });
        },
        // async removeItem(key, _opts) {},
        async getKeys(base, _opts) {
          return await $api("/api/user-storage/get-keys", {
            method: "post",
            body: { base },
          });
        },
        // async clear(base, _opts) {},
        // async dispose() {},
        // async watch(callback) {},
      };
    });

    const sync = createStorage({
      driver: driver({}),
    });

    return { provide: { sync } };
  },
});
