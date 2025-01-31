<script setup lang="ts">
const day = ref(new Date());

const containerRef = useTemplateRef("container");
const caretRef = useTemplateRef("caret");

const scrollHeight = ref(0);

const datePicker = ref(false);

const dark = useDark();

const { d } = useI18n();

const title = d(new Date(), { month: "short", day: "numeric" });

const spaces = await useSpaces();
const schedule = await useSchedule();

// console.log({ spaces: spaces.value });

const todaysSchedule = computed(() => schedule.value[day.value.getDay()]);

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
      <div class="w-16">
        <span
          v-for="hour in 24"
          :key="hour"
          class="m3-label-medium h-12 flex items-start justify-end pr-2"
        >
          {{ $d(Date.UTC(0, 0, 0, hour - 20), { hour: "numeric" }) }}
        </span>
      </div>

      <div class="relative flex-1">
        <m3-theme
          v-for="({ spaceId, from, to }, i) in todaysSchedule"
          :key="i"
          :color="spaces![spaceId]!.color"
          :dark="dark"
          harmonize
          as-child
        >
          <nuxt-link
            :to="`/space?id=${spaceId}`"
            :style="{
              top: `${(from / 60) * (scrollHeight / 24)}px`,
              height: `${(to / 60 - from / 60) * (scrollHeight / 24)}px`,
            }"
            class="absolute cursor-pointer p-2 w-full flex flex-col items-center justify-center rounded-xl bg-m3-primary-container bg-op-90 text-center text-m3-on-primary-container m3-body-small"
          >
            <md-ripple />

            <span class="w-full font-semibold truncate">
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
          class="absolute w-[calc(100%+1rem)] translate-x-[-1rem] border-(b-2 m3-error)"
        />
      </div>
    </div>
  </div>

  <m3-modal-date-picker v-model="datePicker" />
</template>
