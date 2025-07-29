<script setup lang="ts">
defineProps<{ active?: boolean }>();
</script>

<template>
  <div :class="['scrim', { 'scrim--active': active }]" />
</template>

<style lang="scss">
.scrim {
  @apply bg-scrim pointer-events-none absolute inset-0 z-1 h-full w-full opacity-0;

  transition-timing-function: cubic-bezier(0.2, 0, 0, 1);
  transition-duration: 400ms;
  transition-property: opacity;

  &--active {
    @apply pointer-events-unset opacity-50;
  }
}
</style>
