<script setup lang="ts">
import AbstractCard, { type CardProperties } from "./abstract.vue";

defineProps<CardProperties>();
</script>

<template>
  <abstract-card class="m3-elevated-card">
    <md-elevation />

    <slot />
  </abstract-card>
</template>

<style lang="scss">
.m3-elevated-card {
  @apply text-m3-on-surface bg-m3-surface-container-low;

  --md-elevation-level: 1;
}
</style>
