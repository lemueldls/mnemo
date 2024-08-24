<script setup lang="ts">
const emit = defineEmits<{
  (event: "update:modelValue", value: boolean): void;
}>();
const props = defineProps<{ modelValue: boolean }>();
const visible = useVModel(props, "modelValue", emit);

const today = new Date();
const currentDate = ref(today);

const calendar = computed(() => {
  const year = currentDate.value.getFullYear();
  const month = currentDate.value.getMonth();

  const calendar = [];

  const startOfMonth = new Date(year, month, 1).getDay();
  const daysInMonth = new Date(year, month + 1, 0).getDate();

  for (let index = startOfMonth, day = 1; day < daysInMonth; ) {
    const week = [];

    for (; index < 7; index++, day++)
      week[index] = day > daysInMonth ? undefined : day;

    index = 0;
    calendar.push(week);
  }

  return calendar;
});

const { d } = useI18n();

const months = Array.from({ length: 12 }).map((_, month) =>
  d(Date.UTC(0, month + 1), { month: "long" }),
);
const weekdays = Array.from({ length: 7 }).map((_, weekday) =>
  d(Date.UTC(0, 0, weekday + 1), { weekday: "narrow" }),
);
</script>

<template>
  <div
    :class="[
      'm3-modal-date-picker',
      { 'm3-modal-date-picker--hidden': !visible },
    ]"
  >
    <md-elevation />

    <div class="m3-modal-date-picker__header">
      <span class="m3-modal-date-picker__supporting-text">Select Date</span>

      <div class="flex items-center justify-between gap-2">
        <span class="m3-modal-date-picker__heading">
          {{ useShortDate(currentDate) }}
        </span>

        <md-icon-button><md-icon>edit</md-icon></md-icon-button>
      </div>
    </div>

    <md-divider />

    <div class="p-3">
      <div class="mb-1 flex items-center justify-between">
        <md-outlined-select
          :value="currentDate.getMonth()"
          @input="
            currentDate = new Date(
              currentDate.getFullYear(),
              $event.target.value,
            )
          "
        >
          <md-select-option
            :selected="i === currentDate.getMonth()"
            :value="i"
            v-for="(month, i) in months"
            :key="month"
          >
            <span slot="headline">{{ month }}</span>
          </md-select-option>
        </md-outlined-select>

        <div class="flex">
          <md-icon-button
            @click="
              currentDate = new Date(
                currentDate.getFullYear(),
                currentDate.getMonth() - 1,
              )
            "
          >
            <md-icon>keyboard_arrow_left</md-icon>
          </md-icon-button>
          <md-icon-button
            @click="
              currentDate = new Date(
                currentDate.getFullYear(),
                currentDate.getMonth() + 1,
              )
            "
          >
            <md-icon>keyboard_arrow_right</md-icon>
          </md-icon-button>
        </div>
      </div>

      <div class="mb-2 flex flex-col">
        <div class="flex">
          <div
            v-for="weekday in weekdays"
            :key="weekday"
            class="h-10 w-10 flex flex-1 justify-center"
          >
            {{ weekday }}
          </div>
        </div>

        <div v-for="(week, i) in calendar" :key="i" class="grid grid-cols-7">
          <div
            v-for="day in week"
            :key="day"
            class="flex flex-1 justify-center"
            @click="
              currentDate = new Date(
                currentDate.getFullYear(),
                currentDate.getMonth(),
                day,
              )
            "
          >
            <template v-if="day">
              <md-filled-icon-button v-if="day === currentDate.getDate()">
                {{ day }}
              </md-filled-icon-button>
              <md-filled-tonal-icon-button
                v-else-if="
                  day === today.getDate() &&
                  currentDate.getMonth() === today.getMonth() &&
                  currentDate.getFullYear() === today.getFullYear()
                "
              >
                {{ day }}
              </md-filled-tonal-icon-button>
              <md-icon-button v-else>
                {{ day }}
              </md-icon-button>
            </template>
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
.m3-modal-date-picker {
  @apply absolute bg-m3-surface-container-high rounded-xl flex flex-col;

  --md-elevation-level: 3;

  &__supporting-text {
    @apply m3-label-medium text-m3-on-surface-variant;
  }

  &__header {
    @apply flex flex-col gap-9 p-3 pl-6;
  }

  &__heading {
    @apply m3-headline-large;
  }

  // &__date {}

  &--hidden {
    @apply opacity-0 pointer-events-none;
  }

  // &__date {
  //   @apply w-4 h-4 rounded-xl;
  // }
}
</style>
