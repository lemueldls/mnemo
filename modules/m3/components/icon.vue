<script setup lang="ts">
import type { MaterialSymbol } from "material-symbols";

export interface IconProperties {
  name: MaterialSymbol;
  outlined?: boolean;
  rounded?: boolean;
  sharp?: boolean;
  fill?: boolean;
  weight?: number;
  grade?: number;
  opticalSize?: number;
}

const properties = defineProps<IconProperties>();

const style = computed(() =>
  properties.rounded ? "rounded" : properties.sharp ? "sharp" : "outlined",
);
const fill = computed(() => (properties.fill ? 1 : 0));
const weight = computed(() => properties.weight ?? 400);
const grade = computed(() => properties.grade ?? 0);
const opticalSize = computed(() => properties.opticalSize ?? 48);
</script>

<template>
  <span :class="['icon', `material-symbols-${style}`]">{{
    name.replaceAll("-", "_")
  }}</span>
</template>

<style scoped>
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
