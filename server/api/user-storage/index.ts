// import { createH3StorageHandler } from "unstorage/server";

// const storage = hubKV();

// export default createH3StorageHandler(storage, {
//   authorize(req) {
//     // req: { key, type, event }
//     if (req.type === "read" && req.key.startsWith("private:")) {
//       throw new Error("Unauthorized Read");
//     }
//   },
// });

export default defineEventHandler(() => {});
