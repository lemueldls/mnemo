import { isTauri } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";

export function openExternalUrl(url: string) {
  if (isTauri()) openUrl(url);
  else window.open(url, "_blank");
}

export function usePageRouteQuery(name: string, defaultValue: string = "") {
  const route = useRoute();
  const { path: originalPath } = route;

  const routeQuery = ref(defaultValue);

  watchImmediate(
    () => route.query,
    () => {
      if (route.path === originalPath)
        routeQuery.value =
          [route.query?.[name]]?.flat?.()?.[0]?.toString() || defaultValue;
    },
  );

  return routeQuery;
}
