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
  @apply h-14 flex flex-1 flex-col items-center justify-center m3-label-medium cursor-pointer gap-1;

  &__leading {
    @apply relative rounded-2xl flex w-16 h-8 items-center justify-center text-m3-on-surface-variant;
  }

  &__trailing {
    @apply flex text-m3-on-surface-variant;
  }

  &--active &__leading {
    @apply bg-m3-secondary-container text-m3-on-secondary-container;
  }
}
</style>
