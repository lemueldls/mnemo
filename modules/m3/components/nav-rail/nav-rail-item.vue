<script setup lang="ts">
defineProps<{ active?: boolean }>();
</script>

<template>
  <div :class="['m3-nav-rail-item', { 'm3-nav-rail-item--active': active }]">
    <div v-if="$slots.leading" class="m3-nav-rail-item__leading">
      <md-ripple />

      <slot name="leading" />
    </div>

    <slot />
  </div>
</template>

<style lang="scss">
.m3-nav-rail-item {
  @apply h-14 flex flex-col items-center justify-center m3-label-medium cursor-pointer gap-1;

  &__leading {
    @apply relative rounded-2xl flex w-14 h-8 items-center justify-center text-m3-on-surface-variant;
  }

  &__trailing {
    @apply flex text-m3-on-surface-variant;
  }

  &--active &__leading {
    @apply bg-m3-secondary-container text-m3-on-secondary-container;
  }
}
</style>
