<script setup lang="ts"></script>

<template>
  <div class="top-app-bar" data-tauri-drag-region>
    <div
      v-if="$slots.leading"
      class="top-app-bar__leading"
      data-tauri-drag-region
    >
      <slot name="leading" />
    </div>

    <div class="top-app-bar__headline" data-tauri-drag-region>
      <slot />
    </div>

    <div
      v-if="$slots.trailing"
      class="top-app-bar__trailing"
      data-tauri-drag-region
    >
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
