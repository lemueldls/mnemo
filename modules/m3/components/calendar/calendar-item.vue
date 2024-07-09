<script setup lang="ts">
interface CalendarEvent extends Event {
  top: number;
  height: number;
}

defineProps<{ event: CalendarEvent; space: Space }>();

const dark = useDark();
const spaces = listSpaces();
</script>

<template>
  <m3-theme
    :color="spaces[space].color"
    :dark="dark"
    harmonize
    class="absolute w-full"
    :style="{
      top: `calc(${event.top}px + 1.5rem)`,
      height: `${event.height}px`,
    }"
  >
    <div
      class="h-full flex flex-col items-center justify-center rounded-xl bg-m3-primary-container bg-op-90 text-m3-on-primary-container m3-body-medium"
    >
      {{ space }}

      <span v-if="event.height > 50" class="text-m3-on-surface-variant">
        {{ $d(event.from, { hour: "numeric", minute: "numeric" }) }} -
        {{ $d(event.to, { hour: "numeric", minute: "numeric" }) }}
      </span>
    </div>
  </m3-theme>
</template>
