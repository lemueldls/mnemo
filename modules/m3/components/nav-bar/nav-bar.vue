<script setup lang="ts"></script>

<template>
  <div class="m3-nav-bar">
    <div class="m3-nav-bar__inner">
      <slot />
    </div>

    <div v-if="$slots.trailing" class="m3-nav-bar__trailing">
      <slot name="trailing" />
    </div>
  </div>
</template>

<style lang="scss">
.m3-nav-bar {
  @apply h-22;

  &__inner {
    @apply bg-m3-surface-container fixed bottom-0 flex w-full gap-2 px-3 pb-4 pt-3;
  }

  &__trailing {
    @apply flex flex-1 justify-end gap-2;
  }
}
</style>
