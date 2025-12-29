<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    type?: "docked" | "floating";
    orientation?: "horizontal" | "vertical";
    vibrant?: boolean;
  }>(),
  {
    id: undefined,
    class: undefined,
    type: "docked",
    orientation: "horizontal",
    vibrant: false,
  },
);

const floating = computed(() => props.type === "floating");
</script>

<template>
  <div :class="['toolbar', { 'toolbar--floating': floating, 'toolbar--vibrant': vibrant }]">
    <div class="toolbar__inner">
      <md-elevation />

      <slot />
    </div>
  </div>
</template>

<style lang="scss">
@use "sass:map";

@use "@material/web/tokens";

.toolbar {
  @apply absolute bottom-0 flex w-full;

  /* height: calc(4rem + env(safe-area-inset-bottom)); */

  &--floating {
    @apply p-4;
  }

  &__inner {
    @apply bg-surface-container relative flex justify-between p-4;

    --md-elevation-level: 3;
  }

  &--floating &__inner {
    @apply gap-1 p-2;

    $shapes: tokens.md-sys-shape-values();
    $corner-full: map.get($shapes, "corner-full");

    border-radius: $corner-full;
  }

  &--vibrant &__inner {
    @apply bg-primary-container text-on-primary-container;
  }
}
</style>
