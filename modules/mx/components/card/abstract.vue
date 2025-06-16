<script setup lang="ts">
export interface CardProperties {
  elevation?: number;
  disabled?: boolean;
}

defineProps<CardProperties>();
</script>

<template>
  <div class="card">
    <slot />
  </div>
</template>

<style lang="scss">
:where(.card) {
  @apply relative flex flex-col rounded-xl p-4;
}
</style>
