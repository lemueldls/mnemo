import { polarClient } from "@polar-sh/better-auth";
import { createAuthClient } from "better-auth/client";

import type {
  ClientOptions,
  InferSessionFromClient,
  InferUserFromClient,
} from "better-auth/client";

import type { RouteLocationRaw } from "vue-router";

export const useAuth = createSharedComposable(() => {
  const token = useApiToken();

  const client = createAuthClient({
    baseURL: useApiBaseUrl(),
    fetchOptions: {
      auth: {
        type: "Bearer",
        token: () => token.value,
      },
    },
    plugins: [polarClient()],
  });

  const session = ref<InferSessionFromClient<ClientOptions> | null>(null);
  const user = ref<InferUserFromClient<ClientOptions> | null>(null);
  const sessionFetching = ref(false);

  const fetchSession = async () => {
    if (sessionFetching.value) return;

    sessionFetching.value = true;

    let sessionData;
    try {
      const { error, data } = await client.getSession({
        fetchOptions: {
          headers: {
            Authorization: `Bearer ${token.value}`,
          },
        },
      });

      if (error) {
        console.error("Error fetching session:", error);

        return;
      }

      sessionData = data;
    } catch (error) {
      console.error("Even bigger error fetching session:", error);

      return;
    }

    session.value = sessionData?.session || null;
    user.value = sessionData?.user || null;
    sessionFetching.value = false;

    return sessionData;
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
