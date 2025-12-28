import { localDb } from "./db";
import { itemsRefs } from "./refs";

import type { StorageValue } from "unstorage";

export const useSync = createSharedComposable(() => {
  const token = useApiToken().value;
  const url = new URL(`/api/user-storage?token=${token}`, useApiBaseUrl());
  url.protocol = url.protocol === "https:" ? "wss:" : "ws:";

  const { open, close, send } = useApiWebSocket(url, {
    immediate: false,
    async onMessage(_ws, event) {
      const text = await event.text();
      const { key, value, updatedAt } = JSON.parse(text) as {
        key: string;
        value: StorageValue;
        updatedAt: number;
      };

      const meta = await localDb.getMeta(key);

      if (!meta.updatedAt || updatedAt > (meta.updatedAt as number)) {
        await localDb.setItem(key, value);
        await localDb.setMeta(key, { updatedAt });

        const itemRef = await itemsRefs[key];
        if (itemRef) itemRef.setLocal(value);
      }
    },
  });

  const { loggedIn } = useAuth();
  watchImmediate([loggedIn, useOnline()], ([loggedIn, isOnline]) => {
    if (loggedIn && isOnline) open();
    else close();
  });

  return {
    async updateItem(key: string, value: StorageValue, updatedAt: number) {
      await send(JSON.stringify({ key, value, updatedAt }));
    },
  };
});
