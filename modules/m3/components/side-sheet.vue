<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    id?: string;
    class?: string;
    type?: "standard" | "modal" | "auto";
    modelValue?: boolean;
  }>(),
  { id: undefined, class: undefined, type: "auto" },
);

const visible = defineModel<boolean>();

const { large } = useBreakpoints(breakpointsM3);

const modal = computed(() =>
  props.type === "auto" ? !large.value : props.type === "modal",
);
</script>

<template>
  <m3-scrim :active="modal && visible" @click="visible = false" />

  <div
    :id="id"
    :aria-hidden="true"
    :class="[
      props.class,
      'm3-side-sheet',
      'm3-side-sheet--right',
      modal ? 'm3-side-sheet--modal' : 'm3-side-sheet--standard',
      { 'm3-side-sheet--hidden': !visible },
    ]"
  >
    <slot />
  </div>
</template>

<style lang="scss">
.m3-side-sheet {
  @apply max-w-100 flex h-full min-w-64 translate-x-0 flex-col p-3;

  padding-top: calc(0.75rem + env(safe-area-inset-top));

  transition-timing-function: cubic-bezier(0.2, 0, 0, 1);
  transition-duration: 400ms;
  transition-property: transform, translate, width, height;

  &--left {
    @apply left-0;
  }

  &--right {
    @apply right-0;
  }

  &--standard {
    @apply bg-m3-surface;
  }

  &--modal {
    @apply bg-m3-surface-container-low z-1 absolute;
  }

  &--modal#{&}--left {
    @apply rounded-r-xl;
  }

  &--modal#{&}--right {
    @apply rounded-l-xl;
  }

  &--hidden {
    @apply absolute;
  }

  &--hidden#{&}--left {
    @apply translate-x--100%;
  }

  &--hidden#{&}--right {
    @apply translate-x-100%;
  }
}
</style>
