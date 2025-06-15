<script setup lang="ts">
const visible = defineModel<boolean>();

const element = ref<HTMLElement>();

const { top, bottom, left, right } = useScreenSafeArea();

onClickOutside(element, () => {
  visible.value = false;
});
</script>

<template>
  <m3-container
    ref="element"
    :elevation="2"
    :class="['m3-menu', { 'm3-menu--hidden': !visible }]"
  >
    <slot />
  </m3-container>
</template>

<style lang="scss">
.m3-menu {
  @apply rounded-1 text-m3-on-surface bg-m3-surface absolute flex flex-col gap-3 p-3 py-2;

  top: calc(100% + 0.5rem);

  &--hidden {
    @apply hidden;
  }
}
</style>
