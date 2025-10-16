<script setup lang="ts">
import type { CalendarDate } from "@internationalized/date";

const props = defineProps<{
  date: CalendarDate;
  calendarToday: CalendarDate;
  markedDates?: CalendarDate[];
  disableUnmarkedDates?: boolean;
}>();

const selectedDate = defineModel<CalendarDate>({ required: true });

const isDateToday = computed(
  () => props.date.compare(props.calendarToday) === 0,
);

const dateMarks = computed(() =>
  props.markedDates
    ? props.markedDates.filter(
        (enabledDate) => props.date.compare(enabledDate) === 0,
      ).length
    : 0,
);

const isDateMarked = computed(() =>
  props.markedDates
    ? props.markedDates.some(
        (enabledDate) => props.date.compare(enabledDate) === 0,
      )
    : false,
);

const isDateEnabled = computed(() =>
  props.disableUnmarkedDates ? isDateMarked.value : true,
);
</script>

<template>
  <component
    :is="
      date.compare(selectedDate) === 0
        ? 'md-filled-icon-button'
        : isDateToday
          ? 'md-filled-tonal-icon-button'
          : 'md-icon-button'
    "
    :disabled="!isDateEnabled"
    @click="selectedDate = selectedDate.set(date)"
  >
    <span class="font-sans text-sm font-normal">
      {{ date.day }}
    </span>

    <div
      class="absolute -bottom-3.5 left-0 flex w-10 w-full justify-center gap-1"
    >
      <div
        :class="[
          'size-1 rounded',
          isDateToday ? 'bg-on-primary' : 'bg-primary',
        ]"
        v-for="i in dateMarks"
      />
    </div>
  </component>
</template>
