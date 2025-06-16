<script setup lang="ts"></script>

<template>
  <div class="m3-nav-bar">
    <div class="m3-nav-bar__inner">
      <slot />
    </div>
  </div>
</template>

<style lang="scss">
.m3-nav-bar {
  height: calc(5.5rem + env(safe-area-inset-bottom));

  &__inner {
    @apply bg-m3-surface-container fixed bottom-0 flex w-full gap-2 px-3 pt-3;

    padding-bottom: calc(1rem + env(safe-area-inset-bottom));
  }
}
</style>
