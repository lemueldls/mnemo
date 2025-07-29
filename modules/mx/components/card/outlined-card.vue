<script setup lang="ts">
import AbstractCard, { type CardProperties } from "./abstract.vue";

defineProps<CardProperties>();
</script>

<template>
  <abstract-card class="outlined-card">
    <slot />
  </abstract-card>
</template>

<style>
.outlined-card {
  @apply border-outline-variant border-1 border-solid;
}
</style>
