import { serverAuth } from "#imports";

export default defineEventHandler((event) => serverAuth().handler(toWebRequest(event)));
