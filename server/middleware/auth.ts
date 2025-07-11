export default defineEventHandler(async (event) => {
  if (event.method === "OPTIONS") return new Response(null, { status: 204 });

  const url = getRequestURL(event);
  const origin = getRequestHeader(event, "Origin");

  setHeaders(event, {
    "Access-Control-Allow-Origin": origin || url.origin,
    "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type, Authorization, platform",
    "Access-Control-Allow-Credentials": "true",
  });
});
