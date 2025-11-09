import init, { PackageFile, TypstState } from "mnemo-wasm";

export const useTypst = createSharedComposable(
  async () =>
    await init().then(async () => {
      const typstState = new TypstState();

      const fontSets = [
        // [
        //   import("~~/public/fonts/maple/ttf/MapleMono-Regular.ttf?url"),
        //   import("~~/public/fonts/maple/ttf/MapleMono-Italic.ttf?url"),
        //   import("~~/public/fonts/maple/ttf/MapleMono-Bold.ttf?url"),
        //   import("~~/public/fonts/maple/ttf/MapleMono-BoldItalic.ttf?url"),
        // ],
        [
          import("~~/public/fonts/maple/cn/MapleMono-CN-Regular.ttf?url"),
          import("~~/public/fonts/maple/cn/MapleMono-CN-Italic.ttf?url"),
          import("~~/public/fonts/maple/cn/MapleMono-CN-Bold.ttf?url"),
          import("~~/public/fonts/maple/cn/MapleMono-CN-BoldItalic.ttf?url"),
        ],
        [
          // import("~~/public/fonts/NewCMMath-Book.otf?url"),
          import("~~/public/fonts/NewCMMath-Regular.otf?url"),
        ],
      ];

      for (const fontImports of fontSets) {
        await Promise.all(
          fontImports.map(async (fontImport) => {
            const { default: fileUrl } = await fontImport;

            const response = await fetch(fileUrl);
            const buffer = await response.arrayBuffer();
            const bytes = new Uint8Array(buffer);

            typstState.installFont(bytes);
          }),
        );
      }

      return typstState;
    }),
);

export interface TypstPackageSpec {
  namespace: string;
  name: string;
  version: string;
}

export const useInstalledPackages = async (spaceId: MaybeRefOrGetter<string>) =>
  await useStorageItem<TypstPackageSpec[]>(
    () => `spaces/${toValue(spaceId)}/packages.json`,
    [],
  );

export const installTypstPackage = useMemoize(async (pkg: TypstPackageSpec) => {
  const { $api } = useNuxtApp();

  const { spec, files } = await $api("/api/get-package", {
    query: {
      namespace: pkg.namespace || "preview",
      name: pkg.name,
      version: pkg.version,
    },
  });

  const typstState = await useTypst();
  typstState.installPackage(
    spec,
    files.map(
      (file) => new PackageFile(file.path, new Uint8Array(file.content!)),
    ),
  );
});
