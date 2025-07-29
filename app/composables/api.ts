export function useApi() {
  const { $api } = useNuxtApp();

  return $api;
}

export const useApiBaseUrl = createSharedComposable(() => {
  const { apiBaseUrl } = useRuntimeConfig().public;
  const url = apiBaseUrl ? new URL(apiBaseUrl) : useRequestURL();

  return url.origin;
});

export const useApiToken = createSharedComposable(() => {
  const maxAge = 60 * 60 * 24 * 30; // 30 days

  return useCookie(
    "mnemo.session_token",
    import.meta.dev
      ? { sameSite: "lax", secure: false, httpOnly: true, maxAge }
      : { sameSite: "none", secure: true, httpOnly: true, maxAge },
  );
});
