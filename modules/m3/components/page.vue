<template>
  <div class="m3-page">
    <slot />
  </div>
</template>

<style lang="scss">
.m3-page {
  @apply absolute inset-0 w-full h-full overflow-x-hidden overflow-y-auto text-m3-on-background bg-m3-background flex;

  // font-family: "Roboto Flex", sans-serif;
}
</style>
