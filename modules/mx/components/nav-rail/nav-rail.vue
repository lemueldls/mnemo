<script setup lang="ts"></script>

<template>
  <div class="nav-rail">
    <slot />

    <div v-if="$slots.trailing" class="nav-rail__trailing">
      <slot name="trailing" />
    </div>
  </div>
</template>

<style lang="scss">
.nav-rail {
  @apply bg-surface flex h-full w-20 flex-col gap-3 px-3 py-24;

  &__trailing {
    @apply flex flex-1 justify-end gap-2;
  }
}
</style>
