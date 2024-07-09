<script setup lang="ts">
import AbstractCard, { type CardProperties } from "./abstract.vue";

defineProps<CardProperties>();
</script>

<template>
  <abstract-card class="m3-filled-card">
    <slot />
  </abstract-card>
</template>

<style scoped>
.m3-filled-card {
  @apply text-m3-on-surface bg-m3-surface-container-highest;
}
</style>
