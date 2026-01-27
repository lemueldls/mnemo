<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    type?: "basic" | "full-screen" | "auto";
  }>(),
  { type: "auto" },
);

const visible = defineModel<boolean>();

const { medium } = useBreakpoints(breakpointsM3);

const basic = computed(() => (props.type === "auto" ? medium.value : props.type === "basic"));
</script>

<template>
  <mx-scrim :active="visible && basic" @click="visible = false" />

  <mx-container
    :elevation="3"
    :class="[
      'dialog',
      basic ? 'dialog--basic' : 'dialog--full-screen',
      { 'dialog--hidden': !visible },
    ]"
  >
    <div class="dialog__inner">
      <span v-if="$slots.headline" class="dialog__headline">
        <slot name="headline" />
      </span>

      <slot />
    </div>
  </mx-container>
</template>

<style lang="scss">
.dialog {
  @apply pointer-events-none absolute flex h-full w-full items-center justify-center gap-1;

  &__inner {
    @apply pointer-events-auto opacity-100;
  }

  &__headline {
    @apply headline-small text-on-surface text-center;
  }

  &--basic {
    @apply z-1;
  }

  // &--fullscreen {}

  &--basic &__inner {
    @apply bg-surface-container-high flex max-w-140 min-w-70 flex-col rounded-xl p-6 opacity-100;

    transform-origin: center -1rem;
    transform: scaleY(1) translateY(0);

    transition-timing-function: cubic-bezier(0.2, 0, 0, 1);
    transition-duration: 400ms;
    transition-property: opacity, transform;
  }

  &--basic#{&}--hidden &__inner {
    @apply pointer-events-none opacity-0;

    transform: scaleY(0) translateY(-1rem);
    transition-duration: 200ms;
  }
}
</style>
