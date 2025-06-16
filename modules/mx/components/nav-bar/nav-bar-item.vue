<script setup lang="ts">
defineProps<{ active?: boolean }>();
</script>

<template>
  <div :class="['m3-nav-bar-item', { 'm3-nav-bar-item--active': active }]">
    <div v-if="$slots.leading" class="m3-nav-bar-item__leading">
      <md-ripple />

      <slot name="leading" />
    </div>

    <slot />
  </div>
</template>

<style lang="scss">
.m3-nav-bar-item {
  @apply m3-label-medium flex h-14 flex-1 cursor-pointer flex-col items-center justify-center gap-1;

  &__leading {
    @apply text-m3-on-surface-variant relative flex h-8 w-16 items-center justify-center rounded-2xl;
  }

  &__trailing {
    @apply text-m3-on-surface-variant flex;
  }

  &--active &__leading {
    @apply bg-m3-secondary-container text-m3-on-secondary-container;
  }
}
</style>
