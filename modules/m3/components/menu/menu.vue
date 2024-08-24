<script setup lang="ts">
const props = defineProps<{ modelValue?: boolean }>();

const emit = defineEmits<{
  (event: "update:modelValue", value: boolean): void;
}>();
const visible = useVModel(props, "modelValue", emit);

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
  @apply absolute flex flex-col rounded-1 p-3 py-2 gap-3 text-m3-on-surface bg-m3-surface;

  top: calc(100% + 0.5rem);

  &--hidden {
    @apply hidden;
  }
}
</style>
