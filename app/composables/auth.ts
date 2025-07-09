import { createAuthClient } from "better-auth/client";
import { setupBetterAuthTauri } from "@daveyplate/better-auth-tauri";

import type {
  InferSessionFromClient,
  InferUserFromClient,
  ClientOptions,
} from "better-auth/client";
import type { RouteLocationRaw } from "vue-router";

export function useAuth() {
  const { apiBaseUrl } = useRuntimeConfig().public;
  const url = apiBaseUrl ? new URL(apiBaseUrl) : useRequestURL();
  const baseURL = url.origin;

  const headers = import.meta.server ? useRequestHeaders() : undefined;

  const client = createAuthClient({
    baseURL,
    fetchOptions: { headers },
  });

  setupBetterAuthTauri({
    authClient: client,
    scheme: "mnemo",
    debugLogs: true,
    onRequest(href) {
      console.log("Auth request:", href);
    },
    onSuccess(callbackURL) {
      console.log("Auth successful, callback URL:", callbackURL);
      if (callbackURL) navigateTo(callbackURL, { external: true });
    },
    onError(error) {
      throw createError(error);
    },
  });

  const session = useState<InferSessionFromClient<ClientOptions> | null>(
    "auth:session",
    () => null,
  );
  const user = useState<InferUserFromClient<ClientOptions> | null>(
    "auth:user",
    () => null,
  );
  const sessionFetching = import.meta.server
    ? ref(false)
    : useState("auth:sessionFetching", () => false);

  const fetchSession = async () => {
    if (sessionFetching.value) {
      console.log("already fetching session");
      return;
    }
    sessionFetching.value = true;
    const { data } = await client.getSession({
      fetchOptions: {
        headers,
      },
    });
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

      if (redirectTo) await navigateTo(redirectTo);

      return res;
    },
    fetchSession,
    client,
  };
}
