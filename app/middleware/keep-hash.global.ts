export default defineNuxtRouteMiddleware((to, from) => {
  const fromPath = from.fullPath.replace(/#.*$/, "");
  const toPath = to.fullPath.replace(/#.*$/, "");

  if (from.hash) console.log({ fromPath, toPath });

  if (fromPath !== toPath && from.hash && !to.hash)
    return navigateTo({ ...to, hash: from.hash });
});
