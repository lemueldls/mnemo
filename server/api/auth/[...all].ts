import { serverAuth } from "#imports";

export default defineEventHandler((event) => {
  return serverAuth().handler(toWebRequest(event));
});
