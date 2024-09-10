export default defineNitroPlugin(() => {
  sessionHooks.hook("fetch", async (session, event) => {
    // console.log("[fetch]", { session });
  });

  sessionHooks.hook("clear", async (session, event) => {
    // console.log("[clear]", { session });
  });
});
