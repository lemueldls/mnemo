<script setup lang="ts">
import { today, getLocalTimeZone, getDayOfWeek } from "@internationalized/date";

const timeZone = getLocalTimeZone();
const day = ref(today(timeZone));

const containerRef = useTemplateRef("container");
const caretRef = useTemplateRef("caret");

const scrollHeight = ref(0);

const datePicker = ref(false);

const { d, locale } = useI18n();

const title = computed(() =>
  d(day.value.toDate(timeZone), { month: "short", day: "numeric" }),
);

const spaces = await useSpaces();
const schedule = await useSchedule();

const todaysSchedule = computed(
  () => schedule.value[getDayOfWeek(day.value, locale.value)],
);

// useConsole.log({ todaysSchedule });

onMounted(() => {
  const container = containerRef.value!;
  const caret = caretRef.value!;

  scrollHeight.value = container.scrollHeight;

  useIntervalFn(
    () => {
      const now = new Date();

      caret.style.top = `${
        ((now.getHours() + now.getMinutes() / 60) / 24) * scrollHeight.value
      }px`;
    },
    1000 * 60,
    { immediateCallback: true },
  );
});

function nextDay() {
  day.value = day.value.add({ days: 1 });
}

function previousDay() {
  day.value = day.value.subtract({ days: 1 });
}

// const events = computed<CalendarEvent[]>(() =>
//   currentEvents[selectedDay.getDay()].map((event) => {
//     const fromHours = event.from.getHours() + event.from.getMinutes() / 60;
//     const toHours = event.to.getHours() + event.to.getMinutes() / 60;

//     const top = (fromHours / 24) * scrollHeight.value;
//     const height = (toHours - fromHours) * (scrollHeight.value / 24);

//     return { ...event, top, height };
//   }),
// );
</script>

<template>
  <div class="flex h-full flex-1 flex-col gap-4 overflow-hidden">
    <div class="flex">
      <span
        class="text-m3-primary m3-display-small grow-3 flex flex-1 items-center gap-2"
      >
        {{ title }}

        <md-icon-button @click="datePicker = true">
          <md-icon>expand_more</md-icon>
        </md-icon-button>
      </span>

      <div class="grow-2 flex flex-1">
        <md-icon-button @click="previousDay">
          <md-icon>chevron_left</md-icon>
        </md-icon-button>
        <md-icon-button @click="nextDay">
          <md-icon>chevron_right</md-icon>
        </md-icon-button>
      </div>
    </div>

    <div ref="container" class="flex flex-1 gap-4 overflow-auto">
      <div class="w-16">
        <span
          v-for="hour in 24"
          :key="hour"
          class="m3-label-medium flex h-12 items-start justify-end pr-2"
        >
          {{ $d(Date.UTC(0, 0, 0, hour - 20), { hour: "numeric" }) }}
        </span>
      </div>

      <div class="relative flex-1">
        <m3-theme
          v-for="({ spaceId, from, to }, i) in todaysSchedule"
          :key="i"
          :color="spaces![spaceId]!.color"
          harmonize
          as-child
        >
          <nuxt-link
            :to="`/space?id=${spaceId}`"
            :style="{
              top: `${(from / 60) * (scrollHeight / 24)}px`,
              height: `${(to / 60 - from / 60) * (scrollHeight / 24)}px`,
            }"
            class="bg-m3-primary-container bg-op-50 text-m3-on-primary-container m3-body-small absolute flex w-full cursor-pointer flex-col items-center justify-center rounded-xl p-2 text-center"
          >
            <md-ripple />

            <span class="w-full truncate font-semibold">
              {{ spaces![spaceId]!.name }}
            </span>

            <span class="w-full truncate">
              {{
                $d(new Date(0, 0, 0, 0, from), {
                  hour: "numeric",
                  minute: "numeric",
                })
              }}
              -
              {{
                $d(new Date(0, 0, 0, 0, to), {
                  hour: "numeric",
                  minute: "numeric",
                })
              }}
            </span>
          </nuxt-link>
        </m3-theme>

        <div
          ref="caret"
          class="border-(b-2 m3-error) absolute w-[calc(100%+1rem)] translate-x-[-1rem]"
        />
      </div>
    </div>

    <m3-modal-date-picker v-model="datePicker" />
  </div>
</template>
