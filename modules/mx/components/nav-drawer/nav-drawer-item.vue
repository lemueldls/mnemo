<script setup lang="ts">
defineProps<{
  icon?: string;
  active?: boolean;
}>();
</script>

<template>
  <div
    :class="['m3-nav-drawer-item', { 'm3-nav-drawer-item--active': active }]"
  >
    <md-ripple />

    <div v-if="$slots.leading" class="m3-nav-drawer-item__leading">
      <slot name="leading" />
    </div>

    <slot />

    <div v-if="$slots.trailing" class="m3-nav-drawer-item__trailing">
      <slot name="trailing" />
    </div>
  </div>
</template>

<style lang="scss">
.m3-nav-drawer-item {
  @apply m3-label-large rounded-3 relative flex h-14 items-center gap-3 overflow-hidden text-ellipsis whitespace-nowrap p-4;

  border-radius: var(--md-nav-drawer-item-container-shape);

  &__leading {
    @apply text-6 text-m3-on-surface-variant flex;
  }

  &__trailing {
    @apply text-m3-on-surface-variant flex;
  }

  &--active {
    @apply bg-m3-secondary-container;
  }

  &--active &__leading {
    @apply text-m3-on-secondary-container;
  }
}
</style>
