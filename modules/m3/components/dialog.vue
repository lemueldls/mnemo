<script setup lang="ts">
const properties = withDefaults(
  defineProps<{
    type?: "basic" | "full-screen" | "auto";
    modelValue?: boolean;
  }>(),
  { type: "auto" },
);

const emit = defineEmits<{ (event: "update:modelValue"): void }>();
const visible = useVModel(properties, "modelValue", emit);

const { medium } = useBreakpoints(breakpointsM3);

const basic = computed(() =>
  properties.type === "auto" ? medium.value : properties.type === "basic",
);
</script>

<template>
  <m3-scrim :active="visible && basic" @click="visible = false" />

  <m3-container
    :elevation="3"
    :class="[
      'm3-dialog',
      basic ? 'm3-dialog--basic' : 'm3-dialog--full-screen',
      { 'm3-dialog--hidden': !visible },
    ]"
  >
    <div class="m3-dialog__inner">
      <span v-if="$slots.headline" class="m3-dialog__headline">
        <slot name="headline" />
      </span>

      <slot />
    </div>
  </m3-container>
</template>

<style lang="scss">
.m3-dialog {
  @apply absolute flex items-center justify-center w-full h-full gap-1 pointer-events-none;

  &__inner {
    @apply opacity-100 pointer-events-auto;
  }

  &__headline {
    @apply m3-headline-small text-m3-on-surface text-center;
  }

  &--basic {
    @apply z-1;
  }

  // &--fullscreen {}

  &--basic &__inner {
    @apply rounded-xl opacity-100 p-6 flex flex-col min-w-70 max-w-140 bg-m3-surface-container-high;

    transform-origin: center -1rem;
    transform: scaleY(1) translateY(0);

    transition-timing-function: cubic-bezier(0.2, 0, 0, 1);
    transition-duration: 400ms;
    transition-property: opacity, transform;
  }

  &--basic#{&}--hidden &__inner {
    @apply opacity-0 pointer-events-none;

    transform: scaleY(0) translateY(-1rem);
    transition-duration: 200ms;
  }
}
</style>
