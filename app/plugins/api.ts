import { isTauri } from "@tauri-apps/api/core";
import { fetch as tauriFetch } from "@tauri-apps/plugin-http";

import { ofetch, type $Fetch } from "ofetch";

export default defineNuxtPlugin({
  name: "mnemo:api",
  setup(_nuxtApp) {
    const isPlatform = isTauri();

    const headers = new Headers();

    watchImmediate(useApiToken(), (token) => {
      headers.set("Cookie", `mnemo.session_token=${token || ""};`);
    });

    const fetch = isPlatform ? ofetch : ($fetch as $Fetch);

    const api = fetch.create(
      { baseURL: useApiBaseUrl(), headers },
      { fetch: isPlatform ? tauriFetch : undefined },
    );

    return { provide: { api } };
  },
});
