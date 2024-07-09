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
  @apply flex items-center w-full h-16 gap-2 p-2 bg-m3-surface text-m3-on-surface m3-title-large;

  &__leading {
    @apply flex text-m3-on-surface;
  }

  &__headline {
    @apply flex flex-1;
  }

  &__trailing {
    @apply text-m3-on-surface-variant flex gap-2;
  }
}
</style>
