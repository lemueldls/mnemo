<template>
  <div id="space-layout">
    <slot />
  </div>
</template>

<style scoped>
#space-layout {
  @apply absolute inset-0 h-full w-full overflow-hidden;
}
</style>
