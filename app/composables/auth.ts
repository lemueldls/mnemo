import { isTauri } from "@tauri-apps/api/core";
import { fetch as tauriFetch } from "@tauri-apps/plugin-http";
import { createAuthClient } from "better-auth/client";

import type {
  ClientOptions,
  InferSessionFromClient,
  InferUserFromClient,
} from "better-auth/client";

import type { RouteLocationRaw } from "vue-router";

export const useAuth = createSharedComposable(() => {
  const headers = new Headers();

  const token = useApiToken().value;
  headers.append("Cookie", `mnemo.session_token=${token || ""};`);

  const client = createAuthClient({
    baseURL: useApiBaseUrl(),
    fetchOptions: {
      headers,
      customFetchImpl: isTauri() ? tauriFetch : undefined,
    },
  });

  const session = ref<InferSessionFromClient<ClientOptions> | null>(null);
  const user = ref<InferUserFromClient<ClientOptions> | null>(null);
  const sessionFetching = ref(false);

  const fetchSession = async () => {
    if (sessionFetching.value) return;

    sessionFetching.value = true;

    const headers = new Headers();

    const token = useApiToken().value;
    headers.append("Cookie", `mnemo.session_token=${token || ""};`);

    const { error, data } = await client.getSession({
      fetchOptions: { headers },
    });

    if (error) {
      console.error("Error fetching session:", error);

      return;
    }

    session.value = data?.session || null;
    user.value = data?.user || null;
    sessionFetching.value = false;

    return data;
  };

  if (import.meta.client) {
    client.$store.listen("$sessionSignal", async (signal) => {
      if (!signal) return;
      await fetchSession();
    });
  }

  return {
    session,
    user,
    loggedIn: computed(() => !!session.value),
    signIn: client.signIn,
    signUp: client.signUp,
    async signOut({ redirectTo }: { redirectTo?: RouteLocationRaw } = {}) {
      const res = await client.signOut();
      session.value = null;
      user.value = null;

      useApiToken().value = null;

      if (redirectTo) await navigateTo(redirectTo);

      return res;
    },
    fetchSession,
    client,
  };
});
