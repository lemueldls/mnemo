<script setup lang="ts">
import AbstractCard, { type CardProperties } from "./abstract.vue";

defineProps<CardProperties>();
</script>

<template>
  <abstract-card class="filled-card">
    <slot />
  </abstract-card>
</template>

<style scoped>
.filled-card {
  @apply text-on-surface bg-surface-container-highest;
}
</style>
