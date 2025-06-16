<script setup lang="ts"></script>

<template>
  <div class="top-app-bar">
    <div v-if="$slots.leading" class="top-app-bar__leading">
      <slot name="leading" />
    </div>

    <div class="top-app-bar__headline">
      <slot />
    </div>

    <div v-if="$slots.trailing" class="top-app-bar__trailing">
      <slot name="trailing" />
    </div>
  </div>
</template>

<style lang="scss">
.top-app-bar {
  @apply bg-surface text-on-surface title-large flex w-full items-center gap-2 p-3;

  height: calc(4rem + env(safe-area-inset-top));
  padding-top: calc(0.75rem + env(safe-area-inset-top));

  &__leading {
    @apply text-on-surface flex flex-1;
  }

  &__headline {
    @apply flex;
  }

  &__trailing {
    @apply text-on-surface-variant flex flex-1 justify-end gap-2;
  }
}
</style>
