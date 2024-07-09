<script setup lang="ts"></script>

<template>
  <div class="m3-nav-rail">
    <slot />

    <div v-if="$slots.trailing" class="m3-nav-rail__trailing">
      <slot name="trailing" />
    </div>
  </div>
</template>

<style lang="scss">
.m3-nav-rail {
  @apply flex h-full bg-m3-surface w-20 gap-3 flex-col px-3 py-24;

  &__trailing {
    @apply flex gap-2 justify-end flex-1;
  }
}
</style>
