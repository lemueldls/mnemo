export default defineEventHandler(async (event) => {
  setResponseHeader(event, "Cache-Control", "max-age=604800");

  const { namespace, name, version } = getQuery(event);

  const pkg = await $fetch<Blob>(
    `https://packages.typst.org/${namespace}/${name}-${version}.tar.gz`,
  );

  return pkg;
});
