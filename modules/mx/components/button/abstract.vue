<script setup lang="ts">
import type { ContainerProperties } from "../container.vue";

export interface ButtonProperties extends ContainerProperties {
  disabled?: boolean;
}

defineProps<ButtonProperties>();
</script>

<template>
  <m3-container class="m3-button" role="button" ripple>
    <slot />
  </m3-container>
</template>

<style lang="scss">
.m3-button {
  @apply m-1 min-h-10 min-w-10 items-center justify-center rounded-full p-2;
}
</style>
