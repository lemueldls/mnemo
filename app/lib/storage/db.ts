import { createStorage } from "unstorage";
import indexedDbDriver from "unstorage/drivers/indexedb";

export const localDb = createStorage({
  driver: indexedDbDriver({ base: "app:" }),
});
