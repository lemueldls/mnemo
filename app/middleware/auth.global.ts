export default defineNuxtRouteMiddleware((to, from) => {
  const { ready, loggedIn } = useUserSession();

  if (
    loggedIn.value ||
    !ready.value ||
    to.path.startsWith("/login") ||
    to.path.startsWith("/auth")
  )
    return;

  return navigateTo("/login");
});
