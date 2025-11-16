<template>
  <div class="page">
    <slot />

    <notifications />
  </div>
</template>

<style lang="scss">
.page {
  @apply text-on-background bg-background absolute inset-0 flex h-screen w-full overflow-y-auto overflow-x-hidden;
}
</style>
