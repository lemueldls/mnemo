<script setup lang="ts">
import AbstractButton, { type ButtonProperties } from "./abstract.vue";

import type { IconProperties } from "../icon.vue";

export interface IconButtonProperties
  extends IconProperties,
    ButtonProperties {}

const props = defineProps<IconButtonProperties>();
</script>

<template>
  <abstract-button class="m3-icon-button" v-bind="{ ...props }">
    <m3-icon v-bind="{ ...props }" />
  </abstract-button>
</template>

<style lang="scss">
.m3-icon-button {
  @apply text-6 m-1 h-10 w-10 items-center rounded-full;
}
</style>
