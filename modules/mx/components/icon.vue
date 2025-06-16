<script setup lang="ts">
import type { MaterialSymbol } from "material-symbols";

export interface IconProperties {
  name: MaterialSymbol;
  // outlined?: boolean;
  // rounded?: boolean;
  // sharp?: boolean;
  // variant?: "outlined" | "rounded" | "sharp";
  fill?: boolean;
  weight?: number;
  grade?: number;
  opticalSize?: number;
}

const props = withDefaults(defineProps<IconProperties>(), {
  variant: "rounded",
  weight: 400,
  grade: 0,
  opticalSize: 48,
});

// const variant = computed(
//   () =>
//     props.variant ||
//     (props.rounded ? "rounded" : props.sharp ? "sharp" : "outlined"),
// );
const variant = "rounded";

const fill = computed(() => (props.fill ? 1 : 0));
</script>

<template>
  <span :class="['icon', `material-symbols-${variant}`]">
    {{ name.replaceAll("-", "_") }}
  </span>
</template>

<style>
@import "material-symbols/rounded.css";

.icon {
  width: 1em;
  height: 1em;
  font-variation-settings:
    "FILL" v-bind(fill),
    "wght" v-bind(weight),
    "GRAD" v-bind(grade),
    "opsz" v-bind(opticalSize);
}
</style>
