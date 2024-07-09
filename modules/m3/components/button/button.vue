<script setup lang="ts">
defineProps<{
  type?: "elevated" | "filled" | "filled-tonal" | "outlined" | "text";
  icon?: string;
  disabled?: boolean;
}>();
</script>

<template>
  <m3-container
    :class="[
      'm3-button',
      type && `m3-button--${type}`,
      {
        'm3-button--with-content': $slots.default,
        'm3-button--with-icon': icon,
      },
    ]"
    role="button"
    :disabled="disabled"
    ripple
  >
    <span v-if="icon" :class="['m3-button__icon', icon]" />

    <slot />
  </m3-container>
</template>

<style lang="scss">
.m3-button {
  @apply m3-label-large items-center justify-center gap-2 m-1 min-w-10 h-10 rounded-5 p-2;

  &__icon {
    @apply text-6;
  }

  &--with-content {
    @apply px-6;
  }

  &--with-content &__icon {
    @apply text-4.5;
  }

  &--with-content#{&}--with-icon {
    @apply pl-4;
  }

  &--elevated {
    @apply text-m3-primary bg-surface-container-low;
  }

  &--filled {
    @apply bg-m3-primary text-m3-on-primary;
  }

  &--filled-tonal {
    @apply bg-m3-container text-m3-on-secondary-container;
  }

  &--outlined {
    @apply border border-m3-outline text-m3-primary;
  }

  &--text {
    @apply text-m3-primary;
  }
  &--text#{&}--with-content {
    @apply min-w-12 pl-3;
  }
  &--text#{&}--with-content#{&}--with-icon {
    @apply pr-4;
  }

  &.m3-container--disabled {
    @apply bg-opacity-12 text-opacity-38 text-m3-on-surface;
  }
}
</style>
