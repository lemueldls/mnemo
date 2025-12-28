<script setup lang="ts">
import { Primitive, type PrimitiveProps } from "reka-ui";

import type { Rgba } from "~~/modules/mx/types";

interface Props extends PrimitiveProps {
  color: string;
  dark?: boolean;
  harmonize?: boolean;
}

const props = withDefaults(defineProps<Props>(), { dark: undefined });
const { color, dark, harmonize } = toRefs(props);

const globalDark = useDark();
const isDark = computed(() => (dark.value === undefined ? globalDark.value : dark.value));

const parentTheme = computed(() =>
  harmonize.value ? useMaterialTheme()?.value.source : undefined,
);
const theme = computed(() => createTheme(color.value, isDark.value, parentTheme.value));

provide(m3ThemeKey, theme);

// const { palette } = toRefs(theme);
const palette = computed(() => theme.value.palette);

function parse(color: Rgba) {
  return `rgb(${color.r},${color.g},${color.b})`;
}

const selectionBackground = computed(() => {
  const { r, g, b } = palette.value.tertiaryContainer;
  return `rgba(${r},${g},${b},0.5)`;
});
</script>

<template>
  <Primitive
    :as
    :as-child
    class="theme"
    :style="{
      '--md-sys-color-background': parse(palette.background),
      '--md-sys-color-on-background': parse(palette.onBackground),
      '--md-sys-color-surface': parse(palette.surface),
      '--md-sys-color-surface-dim': parse(palette.surfaceDim),
      '--md-sys-color-surface-bright': parse(palette.surfaceBright),
      '--md-sys-color-surface-container-lowest': parse(palette.surfaceContainerLowest),
      '--md-sys-color-surface-container-low': parse(palette.surfaceContainerLow),
      '--md-sys-color-surface-container': parse(palette.surfaceContainer),
      '--md-sys-color-surface-container-high': parse(palette.surfaceContainerHigh),
      '--md-sys-color-surface-container-highest': parse(palette.surfaceContainerHighest),
      '--md-sys-color-on-surface': parse(palette.onSurface),
      '--md-sys-color-surface-variant': parse(palette.surfaceVariant),
      '--md-sys-color-on-surface-variant': parse(palette.onSurfaceVariant),
      '--md-sys-color-inverse-surface': parse(palette.inverseSurface),
      '--md-sys-color-inverse-on-surface': parse(palette.inverseOnSurface),
      '--md-sys-color-outline': parse(palette.outline),
      '--md-sys-color-outline-variant': parse(palette.outlineVariant),
      '--md-sys-color-shadow': parse(palette.shadow),
      '--md-sys-color-scrim': parse(palette.scrim),
      '--md-sys-color-surface-tint': parse(palette.surfaceTint),
      '--md-sys-color-primary': parse(palette.primary),
      '--md-sys-color-on-primary': parse(palette.onPrimary),
      '--md-sys-color-primary-container': parse(palette.primaryContainer),
      '--md-sys-color-on-primary-container': parse(palette.onPrimaryContainer),
      '--md-sys-color-inverse-primary': parse(palette.inversePrimary),
      '--md-sys-color-secondary': parse(palette.secondary),
      '--md-sys-color-on-secondary': parse(palette.onSecondary),
      '--md-sys-color-secondary-container': parse(palette.secondaryContainer),
      '--md-sys-color-on-secondary-container': parse(palette.onSecondaryContainer),
      '--md-sys-color-tertiary': parse(palette.tertiary),
      '--md-sys-color-on-tertiary': parse(palette.onTertiary),
      '--md-sys-color-tertiary-container': parse(palette.tertiaryContainer),
      '--md-sys-color-on-tertiary-container': parse(palette.onTertiaryContainer),
      '--md-sys-color-error': parse(palette.error),
      '--md-sys-color-on-error': parse(palette.onError),
      '--md-sys-color-error-container': parse(palette.errorContainer),
      '--md-sys-color-on-error-container': parse(palette.onErrorContainer),
      '--md-sys-color-primary-fixed': parse(palette.primaryFixed),
      '--md-sys-color-primary-fixed-dim': parse(palette.primaryFixedDim),
      '--md-sys-color-on-primary-fixed': parse(palette.onPrimaryFixed),
      '--md-sys-color-on-primary-fixed-variant': parse(palette.onPrimaryFixedVariant),
      '--md-sys-color-secondary-fixed': parse(palette.secondaryFixed),
      '--md-sys-color-secondary-fixed-dim': parse(palette.secondaryFixedDim),
      '--md-sys-color-on-secondary-fixed': parse(palette.onSecondaryFixed),
      '--md-sys-color-on-secondary-fixed-variant': parse(palette.onSecondaryFixedVariant),
      '--md-sys-color-tertiary-fixed': parse(palette.tertiaryFixed),
      '--md-sys-color-tertiary-fixed-dim': parse(palette.tertiaryFixedDim),
      '--md-sys-color-on-tertiary-fixed': parse(palette.onTertiaryFixed),
      '--md-sys-color-on-tertiary-fixed-variant': parse(palette.onTertiaryFixedVariant),
    }"
  >
    <slot />
  </Primitive>
</template>

<style lang="scss">
.theme {
  @apply caret-primary;

  scrollbar-color: var(--md-sys-color-primary) transparent;

  ::-webkit-scrollbar {
    width: 0;
    background: transparent;
  }

  ::-webkit-scrollbar-thumb {
    @apply rounded-lg;
    background: var(--md-sys-color-primary);
  }

  ::selection {
    @apply text-tertiary;

    background-color: v-bind(selectionBackground);
  }
}
</style>
