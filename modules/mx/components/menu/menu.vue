<script setup lang="ts">
const visible = defineModel<boolean>();

const element = ref<HTMLElement>();

const { top, bottom, left, right } = useScreenSafeArea();

onClickOutside(element, () => {
  visible.value = false;
});
</script>

<template>
  <mx-container
    ref="element"
    :elevation="2"
    :class="['menu', { 'menu--hidden': !visible }]"
  >
    <slot />
  </mx-container>
</template>

<style lang="scss">
.menu {
  @apply rounded-1 text-on-surface bg-surface absolute flex flex-col gap-3 p-3 py-2;

  top: calc(100% + 0.5rem);

  &--hidden {
    @apply hidden;
  }
}
</style>
