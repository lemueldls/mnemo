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
  <m3-outlined-card>
    <!-- <h1 class="m3-title-large">Login</h1> -->

    <md-filled-button @click="login"> Continue with GitHub </md-filled-button>
  </m3-outlined-card>
</template>
