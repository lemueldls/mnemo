export default defineEventHandler(async (event) => {
  setHeaders(event, {
    "Cross-Origin-Resource-Policy": "cross-origin",
    "Cross-Origin-Opener-Policy": "unsafe-none",
    "Cross-Origin-Embedder-Policy": "unsafe-none",
    "Access-Control-Allow-Origin": "*",
  });
});
