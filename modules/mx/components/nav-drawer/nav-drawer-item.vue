<script setup lang="ts">
defineProps<{
  icon?: string;
  active?: boolean;
}>();
</script>

<template>
  <div :class="['nav-drawer-item', { 'nav-drawer-item--active': active }]">
    <md-ripple />

    <div v-if="$slots.leading" class="nav-drawer-item__leading">
      <slot name="leading" />
    </div>

    <slot />

    <div v-if="$slots.trailing" class="nav-drawer-item__trailing">
      <slot name="trailing" />
    </div>
  </div>
</template>

<style lang="scss">
.nav-drawer-item {
  @apply label-large rounded-3 relative flex h-14 items-center gap-3 overflow-hidden text-ellipsis whitespace-nowrap p-4;

  border-radius: var(--md-nav-drawer-item-container-shape);

  &__leading {
    @apply text-6 text-on-surface-variant flex;
  }

  &__trailing {
    @apply text-on-surface-variant flex;
  }

  &--active {
    @apply bg-secondary-container;
  }

  &--active &__leading {
    @apply text-on-secondary-container;
  }
}
</style>
