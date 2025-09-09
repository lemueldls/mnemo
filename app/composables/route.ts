import { isTauri } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";

export function openExternalUrl(url: string) {
  if (isTauri()) openUrl(url);
  else window.open(url, "_blank");
}

export function usePageRouteQuery(name: string, defaultValue: string = "") {
  const route = useRoute();
  const router = useRouter();

  return computed({
    get: () => [route.query[name]].flat()[0]?.toString() || defaultValue,
    set(query: string) {
      console.log({ query: query });
      router.replace({ ...route, query: { ...route.query, [name]: query } });
    },
  });
}
