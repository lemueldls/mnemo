<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";

const { user, clear, openInPopup } = useUserSession();

watchEffect(() => {
  console.log({ user: user.value });
});

const runtimeConfig = useRuntimeConfig();
const { platform, apiBaseUrl } = runtimeConfig.public;

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
    <h3 class="m3-display-small">Sync</h3>

    <md-outlined-card v-if="usage && quota" class="flex flex-col gap-2 p-4">
      <div class="m3-label-large flex justify-between">
        <strong>Local Quota</strong>

        <span>{{ formatBytes(usage) }} / {{ formatBytes(quota) }}</span>
      </div>

      <md-linear-progress :value="usage / quota" />
    </md-outlined-card>

    <span class="text-m3-error">TODO</span>

    <md-filled-tonal-button v-if="user" @click="clear">
      Logout
    </md-filled-tonal-button>
    <md-filled-button v-else @click="login">
      Continue with GitHub
    </md-filled-button>
  </div>
</template>
