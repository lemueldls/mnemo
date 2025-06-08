export default defineNuxtRouteMiddleware((to, from) => {
  if (from.fullPath !== to.fullPath && from.hash && !to.hash)
    return navigateTo({ ...to, hash: from.hash });
});
