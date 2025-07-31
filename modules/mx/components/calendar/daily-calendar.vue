<script setup lang="ts">
import {
  getDayOfWeek,
  getLocalTimeZone,
  today,
  type CalendarDate,
} from "@internationalized/date";

const timeZone = getLocalTimeZone();
const calendarDate = shallowRef<CalendarDate>(today(timeZone));

const containerRef = useTemplateRef("container");
const caretRef = useTemplateRef("caret");

const scrollHeight = ref(0);

const datePickerOpen = ref(false);

const { d, locale } = useI18n();

const title = computed(() =>
  d(calendarDate.value.toDate(timeZone), { month: "short", day: "numeric" }),
);

const spaces = await useSpaces();
const schedule = await useSchedule();

const todaysSchedule = computed(
  () => schedule.value[getDayOfWeek(calendarDate.value, locale.value)],
);

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
  calendarDate.value = calendarDate.value.add({ days: 1 });
}

function previousDay() {
  calendarDate.value = calendarDate.value.subtract({ days: 1 });
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
        class="text-primary display-small grow-3 flex flex-1 items-center gap-2"
      >
        {{ title }}

        <md-icon-button @click="datePickerOpen = true">
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
      <div class="w-12">
        <span
          v-for="hour in 24"
          :key="hour"
          class="label-medium flex h-12 items-start justify-end pr-2"
        >
          {{ d(Date.UTC(0, 0, 0, hour - 20), { hour: "numeric" }) }}
        </span>
      </div>

      <div class="relative flex-1">
        <template v-for="({ spaceId, from, to }, i) in todaysSchedule" :key="i">
          <mx-theme
            v-if="spaces![spaceId]"
            :color="spaces[spaceId].color"
            harmonize
            as-child
          >
            <nuxt-link
              :to="`/space?id=${spaceId}`"
              :style="{
                top: `${(from / 60) * (scrollHeight / 24)}px`,
                height: `${(to / 60 - from / 60) * (scrollHeight / 24)}px`,
              }"
              class="bg-primary-container bg-op-50 text-on-primary-container body-small absolute flex w-full cursor-pointer flex-col items-center justify-center rounded-xl p-2 text-center"
            >
              <md-ripple />

              <span
                class="w-full truncate font-semibold"
                :title="spaces[spaceId].name"
              >
                {{ spaces[spaceId].name }}
              </span>

              <span class="w-full truncate">
                {{
                  d(new Date(0, 0, 0, 0, from), {
                    hour: "numeric",
                    minute: "numeric",
                  })
                }}
                -
                {{
                  d(new Date(0, 0, 0, 0, to), {
                    hour: "numeric",
                    minute: "numeric",
                  })
                }}
              </span>
            </nuxt-link>
          </mx-theme>
        </template>

        <div
          ref="caret"
          class="border-(b-2 error) absolute w-[calc(100%+1rem)] translate-x-[-1rem]"
        />
      </div>
    </div>

    <mx-modal-date-picker
      v-model="datePickerOpen"
      v-model:date="calendarDate"
    />
  </div>
</template>
