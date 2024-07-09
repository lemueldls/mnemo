<script setup lang="ts">
export interface CardProperties {
  elevation?: number;
  disabled?: boolean;
}

defineProps<CardProperties>();
</script>

<template>
  <div class="m3-card">
    <slot />
  </div>
</template>

<style lang="scss">
.m3-card {
  @apply relative rounded-xl p-4 flex flex-col;
}
</style>
