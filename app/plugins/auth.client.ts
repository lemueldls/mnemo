export default defineNuxtPlugin(async (nuxtApp) => {
  if (!nuxtApp.payload.serverRendered) {
    await useAuth().fetchSession();
  }

  nuxtApp.hook("app:mounted", async () => {
    await useAuth().fetchSession();
  });
});
