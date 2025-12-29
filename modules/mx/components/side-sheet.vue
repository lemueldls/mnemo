<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    id?: string;
    class?: string;
    type?: "standard" | "modal" | "auto";
  }>(),
  { id: undefined, class: undefined, type: "auto" },
);

const visible = defineModel<boolean>();

const { large } = useBreakpoints(breakpointsM3);

const modal = computed(() => (props.type === "auto" ? !large.value : props.type === "modal"));
</script>

<template>
  <mx-scrim :active="modal && visible" @click="visible = false" />

  <div
    :id="id"
    :aria-hidden="!visible"
    :class="[
      props.class,
      'side-sheet',
      'side-sheet--right',
      modal ? 'side-sheet--modal' : 'side-sheet--standard',
      { 'side-sheet--hidden': !visible },
    ]"
  >
    <slot />
  </div>
</template>

<style lang="scss">
.side-sheet {
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
    @apply bg-surface;
  }

  &--standard#{&}--left {
    @apply pr-0;
  }

  &--standard#{&}--right {
    @apply pl-0;
  }

  &--modal {
    @apply bg-surface-container-low z-1 absolute;
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
    @apply translate-x-[-100%];
  }

  &--hidden#{&}--right {
    @apply translate-x-[100%];
  }
}
</style>
