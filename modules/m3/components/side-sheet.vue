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

const emit = defineEmits<{
  (event: "update:modelValue", value: boolean): void;
}>();
const visible = useVModel(props, "modelValue", emit);

const { desktop, expanded } = useBreakpoints(breakpointsM3);

const modal = computed(() =>
  props.type === "auto" ? !expanded.value : props.type === "modal",
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
  @apply flex flex-col max-w-100 min-w-64 h-full p-3 translate-x-0;

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
    @apply bg-m3-surface-container-low absolute z-1;
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
