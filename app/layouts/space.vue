<template>
  <div id="space-layout">
    <slot />
  </div>
</template>

<style scoped>
#space-layout {
  @apply absolute inset-0 w-full h-full overflow-hidden;
}
</style>
