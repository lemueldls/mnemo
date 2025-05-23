export default defineEventHandler((event) => {
  return serverAuth().handler(toWebRequest(event));
});
