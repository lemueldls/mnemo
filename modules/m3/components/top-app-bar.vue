<script setup lang="ts"></script>

<template>
  <div class="m3-top-app-bar">
    <div v-if="$slots.leading" class="m3-top-app-bar__leading">
      <slot name="leading" />
    </div>

    <div class="m3-top-app-bar__headline">
      <slot />
    </div>

    <div v-if="$slots.trailing" class="m3-top-app-bar__trailing">
      <slot name="trailing" />
    </div>
  </div>
</template>

<style lang="scss">
.m3-top-app-bar {
  @apply bg-m3-surface text-m3-on-surface m3-title-large flex h-16 w-full items-center gap-2 p-2;

  &__leading {
    @apply text-m3-on-surface flex;
  }

  &__headline {
    @apply flex flex-1;
  }

  &__trailing {
    @apply text-m3-on-surface-variant flex gap-2;
  }
}
</style>
