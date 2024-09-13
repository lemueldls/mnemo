<script setup lang="ts">
definePageMeta({ layout: "auth" });

const pathname = ref();
onMounted(() => {
  pathname.value = window.location.pathname;
});
</script>

<template>
  <m3-elevated-card class="gap-4 w-120">
    <h1 class="m3-title-large">Login</h1>

    {{ pathname }}

    <md-filled-button href="/auth/github">
      Continue with GitHub
    </md-filled-button>

  </m3-elevated-card>
</template>
