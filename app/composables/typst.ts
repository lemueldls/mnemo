import init, { TypstState, PackageFile } from "mnemo-wasm";
import type { Package } from "~~/server/api/list-packages";

const state = init().then(() => new TypstState());
export const useTypst = () => state;

const installedPackages = reactive<Set<string>>(new Set());
export const useInstalledPackages = createSharedComposable(
  () => installedPackages,
);

export async function installTypstPackage(pkg: Package, namespace = "preview") {
  const { $api } = useNuxtApp();

  installedPackages.add(pkg.name);
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
