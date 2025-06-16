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
:where(.m3-card) {
  @apply relative flex flex-col rounded-xl p-4;
}
</style>
