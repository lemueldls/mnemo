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
  @apply justify-center m-1 min-w-10 min-h-10 rounded-full items-center p-2;
}
</style>
