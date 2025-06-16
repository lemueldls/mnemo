<script setup lang="ts">
defineProps<{ active?: boolean }>();
</script>

<template>
  <div :class="['nav-bar-item', { 'nav-bar-item--active': active }]">
    <div v-if="$slots.leading" class="nav-bar-item__leading">
      <md-ripple />

      <slot name="leading" />
    </div>

    <slot />
  </div>
</template>

<style lang="scss">
.nav-bar-item {
  @apply label-medium flex h-14 flex-1 cursor-pointer flex-col items-center justify-center gap-1;

  &__leading {
    @apply text-on-surface-variant relative flex h-8 w-16 items-center justify-center rounded-2xl;
  }

  &__trailing {
    @apply text-on-surface-variant flex;
  }

  &--active &__leading {
    @apply bg-secondary-container text-on-secondary-container;
  }
}
</style>
