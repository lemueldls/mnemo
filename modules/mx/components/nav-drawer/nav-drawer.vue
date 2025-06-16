<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    type?: "standard" | "modal" | "auto";
  }>(),
  { id: undefined, class: undefined, type: "auto" },
);

const visible = defineModel<boolean>();

const { extraLarge } = useBreakpoints(breakpointsM3);

const modal = computed(() =>
  props.type === "auto" ? !extraLarge.value : props.type === "modal",
);

const route = useRoute();
watch(
  () => route.fullPath,
  () => {
    visible.value = extraLarge.value;
  },
);

watchEffect(() => {
  if (props.type === "auto") visible.value = extraLarge.value;
});
</script>

<template>
  <mx-scrim :active="modal && visible" @click="visible = false" />

  <div
    :class="[
      'nav-drawer',
      'nav-drawer--left',
      modal ? 'nav-drawer--modal' : 'nav-drawer--standard',
      { 'nav-drawer--hidden': !visible },
    ]"
    :style="{ '--md-elevation-level': modal ? 1 : 0 }"
  >
    <md-elevation />

    <div class="nav-drawer__inner">
      <slot name="header" />

      <div v-bind="$slots.default" class="nav-drawer__content">
        <slot />
      </div>

      <slot name="actions" />
    </div>
  </div>
</template>

<style lang="scss">
@use "sass:map";
@use "@material/web/tokens";

.nav-drawer {
  @apply z-1 h-full translate-x-0;

  padding-top: env(safe-area-inset-top);

  transition-timing-function: map.get(
    tokens.md-sys-motion-values(),
    "easing-emphasized"
  );
  transition-duration: map.get(
    tokens.md-sys-motion-values(),
    "duration-medium4"
  );
  transition-property: transform, translate;

  &__inner {
    @apply w-90 flex h-full flex-col p-3;
  }

  &__content {
    @apply flex flex-1 flex-col overflow-auto;
  }

  &--left {
    @apply left-0;
  }

  &--right {
    @apply right-0;
  }

  &--standard {
    @apply bg-surface;
  }

  &--modal {
    @apply bg-surface-container-low absolute;
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
