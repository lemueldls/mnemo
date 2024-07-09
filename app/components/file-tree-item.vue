<script setup lang="ts">
defineProps<{ active?: boolean }>();
</script>

<template>
  <div :class="['file-tree-item', { 'file-tree-item--active': active }]">
    <md-ripple />

    <slot />
  </div>
</template>

<style lang="scss">
.file-tree-item {
  @apply relative cursor-pointer h-10 flex items-center rounded-6 px-4 m3-label-large;

  &--active {
    @apply bg-m3-secondary-container;
  }
}
</style>
