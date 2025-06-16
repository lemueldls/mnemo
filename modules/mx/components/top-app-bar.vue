<script setup lang="ts"></script>

<template>
  <div class="m3-top-app-bar">
    <div v-if="$slots.leading" class="m3-top-app-bar__leading">
      <slot name="leading" />
    </div>

    <div class="m3-top-app-bar__headline">
      <slot />
    </div>

    <div v-if="$slots.trailing" class="m3-top-app-bar__trailing">
      <slot name="trailing" />
    </div>
  </div>
</template>

<style lang="scss">
.m3-top-app-bar {
  @apply bg-m3-surface text-m3-on-surface m3-title-large flex w-full items-center gap-2 p-3;

  height: calc(4rem + env(safe-area-inset-top));
  padding-top: calc(0.75rem + env(safe-area-inset-top));

  &__leading {
    @apply text-m3-on-surface flex flex-1;
  }

  &__headline {
    @apply flex;
  }

  &__trailing {
    @apply text-m3-on-surface-variant flex flex-1 justify-end gap-2;
  }
}
</style>
