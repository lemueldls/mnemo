<script setup lang="ts">
defineProps<{ active?: boolean }>();
</script>

<template>
  <div :class="['m3-scrim', { 'm3-scrim--active': active }]" />
</template>

<style lang="scss">
.m3-scrim {
  @apply absolute inset-0 w-full h-full pointer-events-none bg-m3-scrim opacity-0 z-1;

  transition-timing-function: cubic-bezier(0.2, 0, 0, 1);
  transition-duration: 400ms;
  transition-property: background-color;

  &--active {
    @apply opacity-50 pointer-events-unset;
  }
}
</style>
