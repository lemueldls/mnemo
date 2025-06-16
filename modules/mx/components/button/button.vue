<script setup lang="ts">
defineProps<{
  type?: "elevated" | "filled" | "filled-tonal" | "outlined" | "text";
  icon?: string;
  disabled?: boolean;
}>();
</script>

<template>
  <mx-container
    :class="[
      'button',
      type && `button--${type}`,
      {
        'button--with-content': $slots.default,
        'button--with-icon': icon,
      },
    ]"
    role="button"
    :disabled="disabled"
    ripple
  >
    <span v-if="icon" :class="['button__icon', icon]" />

    <slot />
  </mx-container>
</template>

<style lang="scss">
.button {
  @apply label-large rounded-5 m-1 h-10 min-w-10 items-center justify-center gap-2 p-2;

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
    @apply text-primary bg-surface-container-low;
  }

  &--filled {
    @apply bg-primary text-on-primary;
  }

  &--filled-tonal {
    @apply bg-container text-on-secondary-container;
  }

  &--outlined {
    @apply border-outline text-primary border;
  }

  &--text {
    @apply text-primary;
  }
  &--text#{&}--with-content {
    @apply min-w-12 pl-3;
  }
  &--text#{&}--with-content#{&}--with-icon {
    @apply pr-4;
  }

  &.container--disabled {
    @apply bg-opacity-12 text-opacity-38 text-on-surface;
  }
}
</style>
