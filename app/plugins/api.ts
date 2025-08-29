export default defineNuxtPlugin({
  name: "mnemo:api",
  setup(_nuxtApp) {
    const headers = new Headers();

    watchImmediate(useApiToken(), (token) => {
      headers.set("Authorization", `Bearer ${token}`);
    });

    const api = $fetch.create({ baseURL: useApiBaseUrl(), headers });

    return { provide: { api } };
  },
});
