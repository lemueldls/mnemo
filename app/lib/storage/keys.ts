import { localDb } from "./db";

export interface StorageDirPath {
  kind: "directory";
  key: string;
  children: { [key: string]: StoragePath };
}

export interface StorageFilePath {
  kind: "file";
  key: string;
  content: string;
}

export type StoragePath = StorageDirPath | StorageFilePath;

export async function getStorageKeys(base?: string) {
  const localKeys = await localDb.getKeys(base);

  const root: { [key: string]: StoragePath } = {};

  Promise.all(
    localKeys.map(async (localKey) => {
      const paths = localKey.split(":");
      const key = paths.slice(1).join("/");

      let path;
      let directory = root;

      for (let i = 1; i < paths.length; i++) {
        path = paths[i]!;

        if (i < paths.length - 1) {
          directory[path] ||= { kind: "directory", key, children: {} };
          directory = (directory[path] as StorageDirPath).children;
        }
      }

      const content = await localDb.getItem<string>(key);

      directory[path!] ||= { kind: "file", key, content: content! };
    }),
  );

  return root;
}
