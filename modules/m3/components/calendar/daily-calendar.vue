<script setup lang="ts">
const day = ref(new Date());

const container = ref<HTMLDivElement>();
const caret = ref<HTMLDivElement>();

const scrollHeight = ref(0);

const datePicker = ref(false);

onMounted(() => {
  scrollHeight.value = container.value!.scrollHeight;

  useIntervalFn(
    () => {
      if (isToday(day.value)) {
        const now = new Date();

        caret.value!.style.top = `calc(${
          ((now.getHours() + now.getMinutes() / 60) / 24) * scrollHeight.value
        }px + 1.5rem)`;
      }
    },
    1000 * 60,
    { immediateCallback: true },
  );
});

function isToday(date: Date) {
  const now = new Date();

  return (
    date.getFullYear() === now.getFullYear() &&
    date.getMonth() === now.getMonth() &&
    date.getDate() === now.getDate()
  );
}

const dark = useDark();

const { d } = useI18n();

const title = d(new Date(), { month: "short", day: "numeric" });

const selectedDay = new Date();

const spaces = listSpaces();
const { currentEvents } = useEvents();

// console.log({ spaces: spaces.value });

// const currentEvents = computed(() => {
//   return events.filter((event) => isToday(event.from) || isToday(event.to));
// });

const events = computed<CalendarEvent[]>(() =>
  currentEvents[selectedDay.getDay()].map((event) => {
    const fromHours = event.from.getHours() + event.from.getMinutes() / 60;
    const toHours = event.to.getHours() + event.to.getMinutes() / 60;

    const top = (fromHours / 24) * scrollHeight.value;
    const height = (toHours - fromHours) * (scrollHeight.value / 24);

    return { ...event, top, height };
  }),
);
</script>

<template>
  <div class="h-full flex flex-1 flex-col gap-4 overflow-hidden">
    <div class="flex gap-4">
      <span class="flex items-center text-m3-primary m3-display-small">
        {{ title }}

        <md-icon-button @click="datePicker = true">
          <md-icon>expand_more</md-icon>
        </md-icon-button>
      </span>

      <div class="flex">
        <md-icon-button>
          <md-icon>chevron_left</md-icon>
        </md-icon-button>
        <md-icon-button>
          <md-icon>chevron_right</md-icon>
        </md-icon-button>
      </div>
    </div>

    <div ref="container" class="flex flex-1 gap-4 overflow-auto">
      <div class="flex flex-col items-end gap-6 m3-body-large">
        <span v-for="hour in 24" :key="hour">
          {{ $d(Date.UTC(0, 0, 0, hour - 20), { hour: "numeric" }) }}
        </span>
      </div>

      <div class="relative flex-1">
        <!-- <m3-calendar-item
          v-for="(event, i) in events"
          :event="event"
          :space="event.space"
          :key="i"
        /> -->

        <div ref="caret" class="absolute w-full border-(b m3-outline)" />
      </div>
    </div>
  </div>

  <m3-modal-date-picker v-model="datePicker" />
</template>
