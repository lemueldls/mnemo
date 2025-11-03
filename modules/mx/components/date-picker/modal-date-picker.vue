<script setup lang="ts">
import {
  CalendarDate,
  endOfMonth,
  getDayOfWeek,
  startOfMonth,
  today,
} from "@internationalized/date";

import MxModalDatePickerDay from "./modal-date-picker-day.vue";

defineProps<{
  markedDates?: CalendarDate[];
  disableUnmarkedDates?: boolean;
}>();

const visible = defineModel<boolean>();

const { d, locale } = useI18n();

const timeZone = useTimeZone();
const calendarToday = today(timeZone);
const modelDate = defineModel<CalendarDate>("date", { required: true });

const selectedDate = shallowRef(modelDate.value) as Ref<CalendarDate>;
const viewingDate = shallowRef(modelDate.value);

whenever(visible, () => {
  selectedDate.value = modelDate.value;
  viewingDate.value = modelDate.value;
});

function setToday() {
  selectedDate.value = calendarToday;
  viewingDate.value = calendarToday;
}

const year = new Date().getFullYear();

const calendar = computed(() => {
  const date = viewingDate.value;
  const calendar = [];

  const monthStartWeekday = getDayOfWeek(startOfMonth(date), locale.value);
  const monthEndDay = endOfMonth(date).day;

  const month = date.month;

  for (let index = monthStartWeekday, day = 1; day < monthEndDay; ) {
    const week = [];

    for (; index < 7; index++, day++)
      week[index] =
        day > monthEndDay ? undefined : new CalendarDate(year, month, day);

    index = 0;
    calendar.push(week);
  }

  return calendar;
});

const months = computed(() =>
  Array.from({ length: 12 }).map((_, month) =>
    d(Date.UTC(0, month + 1), { month: "long" }),
  ),
);
const weekdays = computed(() =>
  Array.from({ length: 7 }).map((_, weekday) =>
    d(Date.UTC(0, 0, weekday + 1), { weekday: "narrow" }),
  ),
);

function selectDate() {
  modelDate.value = selectedDate.value;
  visible.value = false;
}
</script>

<template>
  <PopoverRoot modal v-model:open="visible">
    <PopoverTrigger>
      <slot />
    </PopoverTrigger>

    <PopoverAnchor />

    <PopoverContent>
      <div
        :class="[
          'modal-date-picker',
          // { 'modal-date-picker--hidden': !visible },
        ]"
      >
        <md-elevation />

        <div class="modal-date-picker__header">
          <span class="modal-date-picker__supporting-text">Select Date</span>

          <div class="flex items-center justify-between">
            <span class="modal-date-picker__heading">
              {{ useShortDate(selectedDate.toDate(timeZone)) }}
            </span>

            <!-- <md-icon-button><md-icon>edit</md-icon></md-icon-button> -->
            <md-text-button @click="setToday">Today</md-text-button>
          </div>
        </div>

        <md-divider />

        <div class="flex flex-col gap-3 p-3">
          <div class="mb-1 flex items-center justify-between">
            <md-outlined-select
              :value="viewingDate.month"
              @input="
                viewingDate = viewingDate.set({
                  month: $event.target.value,
                })
              "
            >
              <md-select-option
                v-for="(month, i) in months"
                :key="month"
                :selected="i + 1 === viewingDate.month"
                :value="i + 1"
              >
                <span slot="headline">{{ month }}</span>
              </md-select-option>
            </md-outlined-select>

            <div class="flex">
              <md-icon-button
                @click="viewingDate = viewingDate.subtract({ months: 1 })"
              >
                <md-icon>keyboard_arrow_left</md-icon>
              </md-icon-button>
              <md-icon-button
                @click="viewingDate = viewingDate.add({ months: 1 })"
              >
                <md-icon>keyboard_arrow_right</md-icon>
              </md-icon-button>
            </div>
          </div>

          <div class="flex flex-col">
            <div class="flex">
              <div
                v-for="weekday in weekdays"
                :key="weekday"
                class="flex size-10 flex-1 items-center justify-center"
              >
                {{ weekday }}
              </div>
            </div>

            <div
              v-for="(week, i) in calendar"
              :key="i"
              class="grid grid-cols-7"
            >
              <div
                v-for="date in week"
                :key="date?.day"
                class="flex flex-1 justify-center"
              >
                <mx-modal-date-picker-day
                  v-if="date"
                  :date
                  :calendar-today
                  :marked-dates
                  :disable-unmarked-dates
                  v-model="selectedDate"
                />
              </div>
            </div>

            <div class="items-center-gap-2 flex justify-end">
              <md-text-button @click="visible = false">Cancel</md-text-button>
              <md-text-button @click="selectDate">OK</md-text-button>
            </div>
          </div>
        </div>
      </div>
    </PopoverContent>
  </PopoverRoot>
</template>

<style lang="scss">
.modal-date-picker {
  @apply bg-surface-container-high flex flex-col rounded-xl;

  --md-elevation-level: 3;

  &__supporting-text {
    @apply label-medium text-on-surface-variant;
  }

  &__header {
    @apply flex flex-col gap-4 p-3 pl-6;
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
