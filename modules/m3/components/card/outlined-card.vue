<script setup lang="ts">
import AbstractCard, { type CardProperties } from "./abstract.vue";

defineProps<CardProperties>();
</script>

<template>
  <abstract-card class="m3-outlined-card">
    <slot />
  </abstract-card>
</template>

<style>
.m3-outlined-card {
  @apply border-1 border-m3-outline border-solid;
}
</style>
