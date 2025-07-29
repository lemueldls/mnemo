<template>
  <div class="page">
    <slot />
  </div>
</template>

<style lang="scss">
.page {
  @apply text-on-background bg-background absolute inset-0 flex h-screen w-full overflow-x-hidden overflow-y-auto;
}
</style>
