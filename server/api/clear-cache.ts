export default defineEventHandler(async (event) => {
  await useStorage("cache").clear();
});
