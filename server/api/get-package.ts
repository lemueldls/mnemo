import { type ParsedTarFileItem, parseTarGzip } from "nanotar";

export default defineCachedEventHandler(
  async (event) => {
    const { namespace = "preview", name, version } = getQuery(event);

    const pkg = await $fetch<Blob>(
      `https://packages.typst.org/${namespace}/${name}-${version}.tar.gz`,
    );

    const buffer = await pkg.arrayBuffer();
    const items = await parseTarGzip(buffer);

    const spec = `@${namespace}/${name}:${version}`;
    const files = items
      .filter((item) => item.type === "file")
      .map((item) => ({
        path: item.name,
        content: Array.from(item.data!),
      }));

    return { spec, files };
  },
  { maxAge: 60 * 60 * 24 * 7, staleMaxAge: -1, getKey: (event) => event.path },
);
