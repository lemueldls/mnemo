<script setup lang="ts">
import type { ContainerProperties } from "../container.vue";

export interface ButtonProperties extends ContainerProperties {
  disabled?: boolean;
}

defineProps<ButtonProperties>();
</script>

<template>
  <mx-container class="button" role="button" ripple>
    <slot />
  </mx-container>
</template>

<style lang="scss">
.button {
  @apply m-1 min-h-10 min-w-10 items-center justify-center rounded-full p-2;
}
</style>
