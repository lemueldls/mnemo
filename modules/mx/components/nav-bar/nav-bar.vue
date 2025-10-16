<script setup lang="ts"></script>

<template>
  <div class="nav-bar">
    <div class="nav-bar__inner">
      <slot />
    </div>
  </div>
</template>

<style lang="scss">
.nav-bar {
  height: calc(4.25rem + env(safe-area-inset-bottom));

  &__inner {
    @apply bg-surface-container fixed bottom-0 flex w-full gap-2 px-3 pt-1.5;

    padding-bottom: calc(0.375rem + env(safe-area-inset-bottom));
  }
}
</style>
