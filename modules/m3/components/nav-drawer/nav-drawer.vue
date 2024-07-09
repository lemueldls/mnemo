<script setup lang="ts">
const properties = withDefaults(
  defineProps<{
    type?: "standard" | "modal" | "auto";
    modelValue?: boolean;
  }>(),
  { id: undefined, class: undefined, type: "auto" },
);

const emits = defineEmits<{
  (event: "update:modelValue", value: boolean): void;
}>();
const visible = useVModel(properties, "modelValue", emits);

const { desktop } = useBreakpoints(breakpointsM3);

const modal = computed(() =>
  properties.type === "auto" ? !desktop.value : properties.type === "modal",
);

const route = useRoute();
watch(
  () => route.href,
  () => {
    visible.value = desktop.value;
  },
);

watchEffect(() => {
  if (properties.type === "auto") visible.value = desktop.value;
});
</script>

<template>
  <m3-scrim :active="modal && visible" @click="visible = false" />

  <div
    :class="[
      'm3-nav-drawer',
      'm3-nav-drawer--left',
      modal ? 'm3-nav-drawer--modal' : 'm3-nav-drawer--standard',
      { 'm3-nav-drawer--hidden': !visible },
    ]"
    :style="{ '--md-elevation-level': modal ? 1 : 0 }"
  >
    <md-elevation />

    <slot />
  </div>
</template>

<style lang="scss">
@use "sass:map";
@use "@material/web/tokens";

.m3-nav-drawer {
  @apply flex flex-col h-full p-3 w-90 translate-x-0 z-1;

  transition-timing-function: map.get(
    tokens.md-sys-motion-values(),
    "easing-emphasized"
  );
  transition-duration: map.get(
    tokens.md-sys-motion-values(),
    "duration-medium4"
  );
  transition-property: transform, translate;

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
    @apply bg-m3-surface-container-low absolute;
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
