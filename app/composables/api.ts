export const useApiBaseUrl = createSharedComposable(() => {
  const { apiBaseUrl } = useRuntimeConfig().public;
  const url = apiBaseUrl ? new URL(apiBaseUrl) : useRequestURL();

  return url.origin;
});

export const useApiSession = createSharedComposable(() => {
  const maxAge = 60 * 60 * 24 * 30; // 30 days

  return useCookie(
    "nuxt-session",
    import.meta.dev
      ? { sameSite: "lax", secure: false, httpOnly: false, maxAge }
      : { sameSite: "none", secure: true, httpOnly: false, maxAge },
  );
});
