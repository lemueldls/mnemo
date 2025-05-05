import init, { TypstState, PackageFile } from "mnemo-wasm";
import type { Package } from "~~/server/api/list-packages";

const state = init().then(() => new TypstState());
export const useTypst = () => state;

export const useInstalledPackages = async (spaceId: string) =>
  await useStorageItem<Package[]>(`spaces/${spaceId}/packages.json`, []);

export function comparePackage(a: Package, b: Package) {
  return a.name === b.name && a.version === b.version;
}

export async function installTypstPackage(pkg: Package, namespace = "preview") {
  const { $api } = useNuxtApp();

  const { spec, files } = await $api("/api/get-package", {
    query: { namespace, name: pkg.name, version: pkg.version },
  });

  const typstState = await useTypst();
  typstState.installPackage(
    spec,
    files.map(
      (file) => new PackageFile(file.path, Uint8Array.from(file.content!)),
    ),
  );
}
