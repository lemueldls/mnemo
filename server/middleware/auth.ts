export default defineEventHandler(async (event) => {
  const url = getRequestURL(event);
  const origin = getRequestHeader(event, "Origin");

  setHeaders(event, {
    "Access-Control-Allow-Origin": origin ?? url.origin,
    "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type, Authorization, User-Agent",
    "Access-Control-Allow-Credentials": "true",
  });

  if (event.method === "OPTIONS") {
    setResponseStatus(event, 204);

    return null;
  }
});
