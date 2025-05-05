<script setup lang="ts">
const color = "#16161d";
</script>

<template>
  <m3-theme :color="color" class="absolute inset-0 h-full">
    <m3-page class="flex flex-1 flex-col items-center justify-center">
      <slot />
    </m3-page>
  </m3-theme>
</template>
