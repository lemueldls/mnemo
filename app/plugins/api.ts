import { isTauri } from "@tauri-apps/api/core";
import { fetch as tauriFetch } from "@tauri-apps/plugin-http";

import { ofetch, type $Fetch } from "ofetch";

export default defineNuxtPlugin({
  name: "mnemo:api",
  setup(_nuxtApp) {
    const isPlatform = isTauri();

    const fetch = (
      import.meta.client ? (isPlatform ? ofetch : useRequestFetch()) : $fetch
    ) as $Fetch;

    const api = fetch.create(
      {
        baseURL: useApiBaseUrl(),
        headers: useRequestHeaders(["cookie"]),
      },
      { fetch: isPlatform ? tauriFetch : undefined },
    );

    return { provide: { api } };
  },
});
