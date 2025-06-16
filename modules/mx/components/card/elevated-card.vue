<script setup lang="ts">
import AbstractCard, { type CardProperties } from "./abstract.vue";

defineProps<CardProperties>();
</script>

<template>
  <abstract-card class="elevated-card">
    <md-elevation />

    <slot />
  </abstract-card>
</template>

<style lang="scss">
.elevated-card {
  @apply text-on-surface bg-surface-container-low;

  --md-elevation-level: 1;
}
</style>
