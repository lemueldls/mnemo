import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";

export async function checkForAppUpdates() {
  const { platform } = useRuntimeConfig().public;
  if (platform !== "windows" && platform !== "darwin" && platform !== "linux") return;

  try {
    const update = await check();
    if (!update) return;

    console.log(`found update ${update.version} from ${update.date} with notes ${update.body}`);

    let downloaded = 0;
    let contentLength = 0;

    // Alternatively, we could also call update.download() and update.install() separately
    await update.downloadAndInstall((event) => {
      switch (event.event) {
        case "Started":
          contentLength = event.data.contentLength!;
          console.log(`started downloading ${event.data.contentLength} bytes`);
          break;
        case "Progress":
          downloaded += event.data.chunkLength;
          console.log(`downloaded ${downloaded} from ${contentLength}`);
          break;
        case "Finished":
          console.log("download finished");
          break;
      }
    });

    console.log("update installed");

    await relaunch();
  } catch (error) {
    console.error("Failed update:", error);
  }
}
