<template>
  <div class="m3-page">
    <slot />
  </div>
</template>

<style lang="scss">
.m3-page {
  @apply text-m3-on-background bg-m3-background absolute inset-0 flex h-full w-full overflow-y-auto overflow-x-hidden;
}
</style>
