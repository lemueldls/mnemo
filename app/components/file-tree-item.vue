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
  @apply rounded-6 m3-label-large relative flex h-10 cursor-pointer items-center px-4;

  &--active {
    @apply bg-m3-secondary-container;
  }
}
</style>
