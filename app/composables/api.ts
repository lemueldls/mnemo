export function useApi() {
  const { $api } = useNuxtApp();

  return $api;
}

export const useApiBaseUrl = createSharedComposable(() => {
  const { apiBaseUrl } = useRuntimeConfig().public;
  const url = apiBaseUrl ? new URL(apiBaseUrl) : useRequestURL();

  return url.origin;
});

export const useApiToken = createSharedComposable(() => useLocalStorage("bearer_token", ""));
