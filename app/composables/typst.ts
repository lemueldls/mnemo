import init, { TypstState, type TypstRequest } from "mnemo-wasm";

export function getTypstFontImports() {
  return [
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
}

export const useTypst = createSharedComposable(
  async () =>
    await init().then(async () => {
      const typstState = new TypstState();

      const fontSets = getTypstFontImports();

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
  await useStorageList<TypstPackageSpec[]>(
    () => `spaces/${toValue(spaceId)}/packages.json`,
    [],
  );

export const installTypstPackage = useMemoize(
  (pkg: TypstPackageSpec, spaceId: string) => {
    const { createNotification } = useNotifications();

    const spec = usePackageSpec(pkg);

    // oxlint-disable-next-line no-async-promise-executor
    return new Promise<void>(async (resolve, reject) => {
      const spaces = await useSpaces();
      const space = spaces.value[spaceId]!;

      const installedPackages = await useInstalledPackages(spaceId);
      const hasPackage = installedPackages.value.some(
        (p) =>
          // p.namespace === pkg.namespace &&
          p.name === pkg.name && p.version === pkg.version,
      );

      if (hasPackage) {
        await loadTypstPackage(pkg).catch(reject);

        return resolve();
      }

      createNotification(
        `${space.name} is requesting to install the package: \`${spec}\``,
        {
          actions: [
            {
              label: "Install",
              variant: "primary",
              async onClick() {
                try {
                  await loadTypstPackage(pkg);

                  const installedPackages = await useInstalledPackages(spaceId);
                  installedPackages.push(pkg);

                  resolve();
                } catch (error) {
                  createNotification(
                    `Error installing ${spec}: ${error instanceof Error ? error.message : String(error)}`,
                    { type: "error" },
                  );

                  reject(error);
                }
              },
            },
          ],
        },
      );
    });
  },
);

async function loadTypstPackage(pkg: TypstPackageSpec) {
  const { $api } = useNuxtApp();

  const data = await $api("/api/download-package", {
    query: pkg,
    responseType: "blob",
  });

  const blob = data as Blob;
  const buffer = await blob.arrayBuffer();

  const typstState = await useTypst();
  typstState.installPackage(usePackageSpec(pkg), new Uint8Array(buffer));
}

export function usePackageSpec(pkg: TypstPackageSpec) {
  return `@${pkg.namespace}/${pkg.name}:${pkg.version}`;
}

export function handleTypstRequests(requests: TypstRequest[], spaceId: string) {
  return Promise.all(
    requests.map(async (request) => {
      switch (request.type) {
        case "source": {
          const path = request.value;

          const typstState = await useTypst();
          const fileId = typstState.createFileId(path);

          break;
        }

        case "package": {
          const pkg = request.value;
          await installTypstPackage(pkg, spaceId);

          break;
        }
      }
    }),
  );
}
