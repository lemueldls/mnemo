<script setup lang="ts">
defineProps<{ active?: boolean }>();
</script>

<template>
  <div :class="['nav-rail-item', { 'nav-rail-item--active': active }]">
    <div v-if="$slots.leading" class="nav-rail-item__leading">
      <md-ripple />

      <slot name="leading" />
    </div>

    <slot />
  </div>
</template>

<style lang="scss">
.nav-rail-item {
  @apply label-medium flex h-14 cursor-pointer flex-col items-center justify-center gap-1;

  &__leading {
    @apply text-on-surface-variant relative flex h-8 w-14 items-center justify-center rounded-2xl;
  }

  &__trailing {
    @apply text-on-surface-variant flex;
  }

  &--active &__leading {
    @apply bg-secondary-container text-on-secondary-container;
  }
}
</style>
