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
      import("~~/public/fonts/new-cm/otf/NewCMMath-Regular.otf?url"),
      import("~~/public/fonts/new-cm/otf/NewCMMath-Bold.otf?url"),
      // import("~~/public/fonts/new-cm/otf/NewCMSansMath-Regular.otf?url"),
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

      const { t } = useSharedI18n();

      createNotification(
        t("composables.typst.package-request", {
          space: space.name,
          package: spec,
        }),
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
                    t("composables.typst.error-installing", {
                      package: spec,
                      error:
                        error instanceof Error ? error.message : String(error),
                    }),
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

export async function loadTypstPackage(pkg: TypstPackageSpec) {
  const { $api } = useNuxtApp();
  const { t } = useSharedI18n();

  const { createNotification, dismiss } = useNotifications();

  const spec = usePackageSpec(pkg);
  const notifId = createNotification(
    t("composables.typst.installing", { package: spec }),
  );

  const data = await $api("/api/download-package", {
    query: pkg,
    responseType: "blob",
  });

  const blob = data as Blob;
  const buffer = await blob.arrayBuffer();

  const typstState = await useTypst();
  typstState.installPackage(spec, new Uint8Array(buffer));

  dismiss(notifId);
}

export function usePackageSpec(pkg: TypstPackageSpec) {
  return `@${pkg.namespace}/${pkg.name}:${pkg.version}`;
}

/** @returns `true` if the typst state has been updated */
export async function handleTypstRequests(
  requests: TypstRequest[],
  spaceId: string,
) {
  const resolvedRequests = await Promise.all(
    requests.map((request) => handleTypstRequest(request, spaceId)),
  );

  return resolvedRequests.some((update) => update);
}

/** @returns `true` if the typst state has been updated */
const handleTypstRequest = useMemoize(
  async (request: TypstRequest, spaceId: string) => {
    switch (request.type) {
      case "source": {
        const path = request.value;

        const item = await getStorageItem<string>(path);

        if (item) {
          const typstState = await useTypst();
          const fileId = typstState.createSourceId(path, spaceId);

          typstState.insertSource(fileId, item);

          return true;
        }

        return false;
      }

      case "file": {
        const path = request.value;

        const item = await getStorageItem<string>(path);

        if (item) {
          const typstState = await useTypst();
          const fileId = typstState.createFileId(path);

          try {
            typstState.insertFile(fileId, Uint8Array.fromBase64(item));
          } catch {
            const encoder = new TextEncoder();
            typstState.insertFile(fileId, encoder.encode(JSON.stringify(item)));
          }

          return true;
        }

        return false;
      }

      case "package": {
        const pkg = request.value;
        await installTypstPackage(pkg, spaceId);

        return true;
      }
    }
  },
);
