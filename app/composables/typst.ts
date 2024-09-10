import init, { TypstState, PackageFile } from "mnemo-wasm";
import type { Package } from "~~/server/api/list-packages";

// eslint-disable-next-line unicorn/prefer-top-level-await
const state = init().then(() => new TypstState());

export const useTypst = () => state;

export async function installTypstPackage(pkg: Package, namespace = "preview") {
  const { $api } = useNuxtApp();

  const { spec, files } = await $api("/api/get-package", {
    query: { namespace, name: pkg.name, version: pkg.version },
  });

  console.log({ spec, files });

  const typstState = await useTypst();
  typstState.installPackage(
    spec,
    files.map(
      (file) => new PackageFile(file.path, Uint8Array.from(file.content!))
    )
  );
}
