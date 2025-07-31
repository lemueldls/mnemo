<script setup lang="ts">
import {
  endOfMonth,
  getDayOfWeek,
  getLocalTimeZone,
  startOfMonth,
  today,
  type CalendarDate,
} from "@internationalized/date";

const visible = defineModel<boolean>();

const { d, locale } = useI18n();

const timeZone = getLocalTimeZone();
const calendarToday = today(timeZone);
const currentDate = defineModel<CalendarDate>("date", { required: true });

const calendar = computed(() => {
  const calendar = [];

  const monthStartWeekday = getDayOfWeek(
    startOfMonth(currentDate.value),
    locale.value,
  );
  const monthEndDay = endOfMonth(currentDate.value).day;

  for (let index = monthStartWeekday, day = 1; day < monthEndDay; ) {
    const week = [];

    for (; index < 7; index++, day++)
      week[index] = day > monthEndDay ? undefined : day;

    index = 0;
    calendar.push(week);
  }

  return calendar;
});

const months = Array.from({ length: 12 }).map((_, month) =>
  d(Date.UTC(0, month + 1), { month: "long" }),
);
const weekdays = Array.from({ length: 7 }).map((_, weekday) =>
  d(Date.UTC(0, 0, weekday + 1), { weekday: "narrow" }),
);
</script>

<template>
  <div
    :class="['modal-date-picker', { 'modal-date-picker--hidden': !visible }]"
  >
    <md-elevation />

    <div class="modal-date-picker__header">
      <span class="modal-date-picker__supporting-text">Select Date</span>

      <div class="flex items-center justify-between gap-2">
        <span class="modal-date-picker__heading">
          {{ useShortDate(currentDate.toDate(timeZone)) }}
        </span>

        <!-- <md-icon-button><md-icon>edit</md-icon></md-icon-button> -->
        <md-text-button @click="currentDate = calendarToday"
          >Today</md-text-button
        >
      </div>
    </div>

    <md-divider />

    <div class="p-3">
      <div class="mb-1 flex items-center justify-between">
        <md-outlined-select
          :value="currentDate.month"
          @input="currentDate = currentDate.set({ month: $event.target.value })"
        >
          <md-select-option
            v-for="(month, i) in months"
            :key="month"
            :selected="i === currentDate.month"
            :value="i"
          >
            <span slot="headline">{{ month }}</span>
          </md-select-option>
        </md-outlined-select>

        <div class="flex">
          <md-icon-button
            @click="currentDate = currentDate.subtract({ months: 1 })"
          >
            <md-icon>keyboard_arrow_left</md-icon>
          </md-icon-button>
          <md-icon-button @click="currentDate = currentDate.add({ months: 1 })">
            <md-icon>keyboard_arrow_right</md-icon>
          </md-icon-button>
        </div>
      </div>

      <div class="mb-2 flex flex-col">
        <div class="flex">
          <div
            v-for="weekday in weekdays"
            :key="weekday"
            class="flex h-10 w-10 flex-1 justify-center"
          >
            {{ weekday }}
          </div>
        </div>

        <div v-for="(week, i) in calendar" :key="i" class="grid grid-cols-7">
          <div
            v-for="day in week"
            :key="day"
            class="flex flex-1 justify-center"
          >
            <component
              :is="
                day === currentDate.day
                  ? 'md-filled-icon-button'
                  : day === calendarToday.day &&
                      currentDate.month === calendarToday.month &&
                      currentDate.year === calendarToday.year
                    ? 'md-filled-tonal-icon-button'
                    : 'md-icon-button'
              "
              v-if="day"
              @click="currentDate = currentDate.set({ day })"
            >
              {{ day }}
            </component>
          </div>
        </div>

        <div class="items-center-gap-2 flex justify-end">
          <md-text-button @click="visible = false">Cancel</md-text-button>
          <md-text-button @click="visible = false">OK</md-text-button>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss">
.modal-date-picker {
  @apply bg-surface-container-high absolute flex flex-col rounded-xl;

  --md-elevation-level: 3;

  &__supporting-text {
    @apply label-medium text-on-surface-variant;
  }

  &__header {
    @apply flex flex-col gap-9 p-3 pl-6;
  }

  &__heading {
    @apply headline-large;
  }

  // &__date {}

  &--hidden {
    @apply pointer-events-none opacity-0;
  }

  // &__date {
  //   @apply w-4 h-4 rounded-xl;
  // }
}
</style>
