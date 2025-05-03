<script setup lang="ts">
definePageMeta({ layout: "auth" });

const { openInPopup } = useUserSession();
</script>

<template>
  <m3-elevated-card class="w-120 gap-4">
    <h1 class="m3-title-large">Login</h1>

    <md-filled-button @click="openInPopup('/auth/github')">
      Continue with GitHub
    </md-filled-button>
  </m3-elevated-card>
</template>
