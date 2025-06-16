<script setup lang="ts">
interface CalendarEvent extends Event {
  top: number;
  height: number;
}

defineProps<{ event: CalendarEvent; space: Space }>();

const spaces = useSpaces();
</script>

<template>
  <mx-theme
    :color="spaces[space].color"
    harmonize
    class="absolute w-full"
    :style="{
      top: `calc(${event.top}px + 1.5rem)`,
      height: `${event.height}px`,
    }"
  >
    <div
      class="bg-primary-container bg-op-90 text-on-primary-container body-medium flex h-full flex-col items-center justify-center rounded-xl"
    >
      {{ space }}

      <span v-if="event.height > 50" class="text-on-surface-variant">
        {{ $d(event.from, { hour: "numeric", minute: "numeric" }) }} -
        {{ $d(event.to, { hour: "numeric", minute: "numeric" }) }}
      </span>
    </div>
  </mx-theme>
</template>
