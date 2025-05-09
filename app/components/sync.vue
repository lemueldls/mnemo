<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";

const runtimeConfig = useRuntimeConfig();
const { platform, apiBaseUrl } = runtimeConfig.public;

const { openInPopup } = useUserSession();

function login() {
  const authPath = "/auth/github";
  if (platform) {
    openUrl(new URL(authPath, apiBaseUrl).toString());
  } else {
    openInPopup(authPath);
  }
}
</script>

<template>
  <h3 class="m3-display-small">Sync</h3>

  <div class="text-m3-error p-4">TODO</div>

  <md-filled-button disabled @click="login">
    Continue with GitHub
  </md-filled-button>
</template>
