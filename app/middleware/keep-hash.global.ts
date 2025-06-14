export default defineNuxtRouteMiddleware((to, from) => {
  if (from.path !== to.path && from.hash && !to.hash)
    return navigateTo({ ...to, hash: from.hash });
});
