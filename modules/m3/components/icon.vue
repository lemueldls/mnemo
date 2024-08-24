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

const props = defineProps<IconProperties>();

const style = computed(() =>
  props.rounded ? "rounded" : props.sharp ? "sharp" : "outlined",
);
const fill = computed(() => (props.fill ? 1 : 0));
const weight = computed(() => props.weight ?? 400);
const grade = computed(() => props.grade ?? 0);
const opticalSize = computed(() => props.opticalSize ?? 48);
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
