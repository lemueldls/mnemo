<script setup lang="ts">
defineProps<{ active?: boolean }>();
</script>

<template>
  <div :class="['m3-scrim', { 'm3-scrim--active': active }]" />
</template>

<style lang="scss">
.m3-scrim {
  @apply bg-m3-scrim z-1 pointer-events-none absolute inset-0 h-full w-full opacity-0;

  transition-timing-function: cubic-bezier(0.2, 0, 0, 1);
  transition-duration: 400ms;
  transition-property: background-color;

  &--active {
    @apply pointer-events-unset opacity-50;
  }
}
</style>
