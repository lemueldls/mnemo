import init, { TypstState, PackageFile } from "mnemo-wasm";
import type { Package } from "~~/server/api/list-packages";

export const useTypst = createSharedComposable(
  async () =>
    await init().then(async () => {
      const typstState = new TypstState();

      const fontImports = [
        import("~~/public/fonts/maple/cn/MapleMono-CN-Regular.ttf?url"),
        import("~~/public/fonts/maple/cn/MapleMono-CN-Italic.ttf?url"),
        import("~~/public/fonts/maple/cn/MapleMono-CN-Bold.ttf?url"),
        import("~~/public/fonts/maple/cn/MapleMono-CN-BoldItalic.ttf?url"),
      ];

      await Promise.all(
        fontImports.map(async (fontImport) => {
          const { default: fileUrl } = await fontImport;

          const response = await fetch(fileUrl);
          const buffer = await response.arrayBuffer();
          const bytes = new Uint8Array(buffer);

          typstState.installFont(bytes);
        }),
      );

      return typstState;
    }),
);

export const useInstalledPackages = async (spaceId: string) =>
  await useStorageItem<Package[]>(`spaces/${spaceId}/packages.json`, []);

export function isSamePackage(a: Package, b: Package) {
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
