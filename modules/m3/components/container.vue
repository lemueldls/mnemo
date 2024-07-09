<script setup lang="ts">
export interface ContainerProperties {
  outlineSize?: string;
  ripple?: boolean;
  active?: boolean;
  disabled?: boolean;
  elevation?: number; // 0 | 1 | 2 | 3 | 4 | 5;
}

const properties = withDefaults(defineProps<ContainerProperties>(), {
  outlineSize: "0",
  ripple: false,
  disabled: false,
  elevation: 0,
});

const keyBoxShadow = computed(
  () =>
    [
      "0px 0px 0px",
      "1px 2px 0px",
      "1px 2px 0px",
      "1px 3px 0px",
      "2px 3px 0px",
      "4px 4px 0px",
    ][properties.elevation],
);

const ambientBoxShadow = computed(
  () =>
    [
      "0px 0px 0px",
      "1px 3px 1px",
      "2px 6px 2px",
      "4px 8px 3px",
      "6px 10px 4px",
      "8px 12px 6px",
    ][properties.elevation],
);

const stateLayer = ref<HTMLElement>();
const { isOutside, elementX, elementY, elementWidth, elementHeight } =
  useMouseInElement(stateLayer);

async function handleClick() {
  if (!stateLayer.value) return;

  const ripple = document.createElement("div");
  ripple.classList.add("m3-container__ripple");

  stateLayer.value.append(ripple);

  const x = isOutside.value ? elementWidth.value / 2 : elementX.value;
  const y = isOutside.value ? elementHeight.value / 2 : elementY.value;

  const rippleIn = useAnimate(
    ripple,
    [{ clipPath: `circle(0% at ${x}px ${y}px)` }, { clipPath: "circle(100%)" }],
    200,
  );
  await until(rippleIn.playState).toBe("finished");

  const fadeOut = useAnimate(ripple, [{ opacity: 0 }], { duration: 200 });
  await until(fadeOut.playState).toBe("finished");

  ripple.remove();
}

const container = ref<HTMLElement>();
const pressed = ref(false);

function handleKeyStroke() {
  if (properties.ripple) {
    pressed.value = !pressed.value;
    container.value?.click();
  }
}

onKeyStroke("Enter", handleKeyStroke, { target: container });
onKeyStroke(" ", handleKeyStroke, { target: container, eventName: "keyup" });
</script>

<template>
  <div
    ref="container"
    :class="[
      'm3-container',
      { 'm3-container--active': active, 'm3-container--disabled': disabled },
    ]"
    :tabindex="ripple && !disabled ? 0 : -1"
    :disabled="disabled"
    :aria-pressed="pressed"
    @click="handleClick"
    @blur="pressed = false"
  >
    <div
      v-if="ripple && !disabled"
      ref="stateLayer"
      class="m3-container__state-layer"
    />

    <slot />
  </div>
</template>

<style lang="scss">
.m3-container {
  @apply flex relative;

  transition-timing-function: cubic-bezier(0.2, 0, 0, 1);
  transition-duration: 50ms;
  transition-property: width, height, background-color;

  &::before,
  &::after {
    @apply pointer-events-none content-empty absolute inset-0 w-full h-full rounded-inherit;
  }

  &::before {
    opacity: 0.03;
    box-shadow: 0 v-bind(keyBoxShadow) var(--md-sys-color-shadow);
  }

  &::after {
    opacity: 0.15;
    box-shadow: 0 v-bind(ambientBoxShadow) var(--md-sys-color-shadow);
  }

  &__state-layer {
    @apply bg-opacity-0 absolute inset-0 bg-m3-on-surface rounded-inherit overflow-hidden;

    transition-duration: 50ms;
    transition-property: background-color;
  }

  &--active:hover > &__state-layer {
    @apply bg-m3-on-secondary-container;
  }

  &__ripple {
    @apply opacity-12 absolute inset-0 bg-m3-on-surface;

    transition-property: opacity;
  }

  &:hover > &__state-layer {
    @apply bg-opacity-8;
  }

  &:focus > &__state-layer {
    @apply bg-opacity-12;
  }
}
</style>
