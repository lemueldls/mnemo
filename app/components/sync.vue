<script setup lang="ts">
import { isTauri } from "@tauri-apps/api/core";
import { onOpenUrl } from "@tauri-apps/plugin-deep-link";
import { openUrl } from "@tauri-apps/plugin-opener";

const runtimeConfig = useRuntimeConfig();
const { platform, apiBaseUrl } = runtimeConfig.public;

const { user, clear, openInPopup } = useUserSession();

if (isTauri()) {
  const unlisten = await onOpenUrl((urls) => {
    const [callback] = urls;
    if (!callback)
      throw createError({
        statusCode: 400,
        statusMessage: "No callback URL provided",
      });

    const callbackUrl = new URL(callback);
    const session = callbackUrl.searchParams.get("session");
    if (!session)
      throw createError({
        statusCode: 400,
        statusMessage: "No session ID provided",
      });

    const maxAge = 60 * 60 * 24 * 7 * 4 * 4; // 4 months
    const cookie = useCookie(
      "nuxt-session",
      import.meta.dev
        ? { sameSite: "lax", secure: false, httpOnly: false, maxAge }
        : { sameSite: "none", secure: true, httpOnly: false, maxAge },
    );

    cookie.value = decodeURIComponent(session);
    window.location.reload();
  });

  onUnmounted(() => {
    unlisten();
  });
}

async function login() {
  if (platform) {
    const baseUrl = apiBaseUrl ? new URL(apiBaseUrl) : useRequestURL();
    await openUrl(new URL("/auth/github", baseUrl));
  } else openInPopup("/auth/github");
}

const quota = ref<number>();
const usage = ref<number>();

onMounted(async () => {
  if (navigator.storage) {
    const estimate = await navigator.storage.estimate();
    quota.value = estimate.quota;
    usage.value = estimate.usage;
  }
});

const units = ["B", "KB", "MB", "GB", "TB", "PB"];
function formatBytes(bytes: number) {
  let i = 0;
  while (bytes >= 1024 && i < units.length - 1) {
    bytes /= 1024;
    i++;
  }

  return `${bytes.toFixed(2)} ${units[i]}`;
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <md-outlined-card v-if="usage && quota" class="flex flex-col gap-2 p-4">
      <div class="label-large flex justify-between">
        <strong>{{ $t("components.sync.local-quota") }}</strong>

        <span>{{ formatBytes(usage) }} / {{ formatBytes(quota) }}</span>
      </div>

      <md-linear-progress :value="usage / quota" />
    </md-outlined-card>

    <md-filled-tonal-button v-if="user" @click="clear">
      {{ $t("components.sync.logout") }}
    </md-filled-tonal-button>
    <md-filled-button v-else @click="login">
      {{ $t("components.sync.continue-with-github") }}
    </md-filled-button>
  </div>
</template>
