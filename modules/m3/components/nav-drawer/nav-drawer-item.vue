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
  @apply relative m3-label-large rounded-3 flex items-center gap-3 p-4 h-14 text-ellipsis whitespace-nowrap overflow-hidden;

  border-radius: var(--md-nav-drawer-item-container-shape);

  &__leading {
    @apply flex text-6 text-m3-on-surface-variant;
  }

  &__trailing {
    @apply flex text-m3-on-surface-variant;
  }

  &--active {
    @apply bg-m3-secondary-container;
  }

  &--active &__leading {
    @apply text-m3-on-secondary-container;
  }
}
</style>
