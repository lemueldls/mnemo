<script setup lang="ts">
defineProps<{ compress?: boolean }>();

const { y } = useWindowScroll();
</script>

<template>
  <header
    :class="['top-app-bar', { 'top-app-bar--compress': compress, 'top-app-bar--on-scroll': y > 0 }]"
    data-tauri-drag-region
  >
    <md-elevation />

    <div class="top-app-bar__leading" data-tauri-drag-region>
      <slot name="leading" />
    </div>

    <div class="top-app-bar__headline" data-tauri-drag-region>
      <slot />
    </div>

    <div v-if="$slots.trailing" class="top-app-bar__trailing" data-tauri-drag-region>
      <slot name="trailing" />
    </div>
  </header>
</template>

<style lang="scss">
.top-app-bar {
  @apply bg-surface text-on-surface title-large duration-250 relative flex w-full items-center gap-2 p-3 transition-colors;

  height: calc(4rem + env(safe-area-inset-top));
  padding-top: calc(0.75rem + env(safe-area-inset-top));

  &--on-scroll {
    @apply text-on-surface-variant bg-surface-variant dark:bg-surface-container;

    --md-elevation-level: 2;
  }

  &__leading {
    @apply text-on-surface flex flex-1;
  }

  &__headline {
    @apply flex;
  }

  &__trailing {
    @apply text-on-surface-variant flex flex-1 justify-end gap-2;
  }

  &--compress &__headline {
    @apply opacity-0 transition-opacity;
  }

  &--compress#{&}--on-scroll &__headline {
    @apply opacity-100;
  }
}
</style>
