export default defineNuxtPlugin({
  name: "mnemo:api",
  setup(_nuxtApp) {
    const runtimeConfig = useRuntimeConfig();
    const { apiBaseUrl } = runtimeConfig.public;

    // if (!apiBaseUrl)
    //   throw createError({ message: "NUXT_PUBLIC_API_BASE_URL is not set" });

    const fetch = import.meta.client
      ? (useRequestFetch() as typeof $fetch)
      : $fetch;
    const api = fetch.create({
      baseURL: apiBaseUrl,
      headers: useRequestHeaders(["cookie"]),
      // onRequest({ request, options, error }) {
      //   if (session.value?.token) {
      //     const headers = (options.headers ||= {});
      //     if (Array.isArray(headers)) {
      //       headers.push(["Authorization", `Bearer ${session.value?.token}`]);
      //     } else if (headers instanceof Headers) {
      //       headers.set("Authorization", `Bearer ${session.value?.token}`);
      //     } else {
      //       headers.Authorization = `Bearer ${session.value?.token}`;
      //     }
      //   }
      // },
      // async onResponseError({ response }) {
      //   if (response.status === 401) {
      //     await nuxtApp.runWithContext(() => navigateTo("/login"));
      //   }
      // },
    });

    return { provide: { api } };
  },
});
