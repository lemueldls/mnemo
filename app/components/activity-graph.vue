<script setup lang="ts">
import {
  CalendarDate,
  getDayOfWeek,
  getLocalTimeZone,
  today,
} from "@internationalized/date";

const container = useTemplateRef("container");
const { width } = useElementSize(container);

const cellSize = 12;
const gapSize = 4;

interface ActivityNode {
  activity: number;
  date: CalendarDate;
}

const graph = shallowRef<ActivityNode[]>([]);
const weightedMax = ref(12);

const timeZone = getLocalTimeZone();
let deltaDate = today(timeZone).subtract({ days: 700 });

const { d, locale } = useI18n();

watchImmediate(width, (width) => {
  const columns = Math.max(Math.floor(width / (cellSize + gapSize)), 16);

  const activities = [];
  for (let x = 0; x < columns; x++) {
    for (let y = 0; y < 7; y++) {
      const date = deltaDate;
      deltaDate = deltaDate.add({ days: 1 });

      const dow = getDayOfWeek(date, locale.value);
      const activity = Math.round(
        Math.random() * (dow == 0 || dow == 6 ? 1 : 12),
      );

      activities.push({ activity, date });
    }
  }

  // const flatActivity = rows.flat();
  // const average = flatActivity.reduce((a, b) =>a + b) / flatActivity.length
  // weightedMax.value = Math.max(8, ...flatActivity);

  graph.value = activities;
});

const months = computed(() =>
  Array.from({ length: 12 }).map((_, month) =>
    d(Date.UTC(0, month + 1), { month: "long" }),
  ),
);
const weekdays = computed(() =>
  Array.from({ length: 7 }).map((_, weekday) =>
    d(Date.UTC(0, 0, weekday + 1), { weekday: "short" }),
  ),
);
</script>

<template>
  <md-outlined-card class="flex flex-col gap-3 overflow-hidden p-3">
    <!-- <div class="flex justify-between">
      <h3 class="title-large text-on-surface-variant">Activity Graph</h3>

      <div class="flex items-center gap-1">
        <span>Less</span>
        <div class="bg-surface-container-low size-3 overflow-hidden rounded-sm">
          <div class="bg-primary size-full opacity-0" />
        </div>
        <div class="bg-surface-container-low size-3 overflow-hidden rounded-sm">
          <div class="bg-primary size-full opacity-25" />
        </div>
        <div class="bg-surface-container-low size-3 overflow-hidden rounded-sm">
          <div class="bg-primary size-full opacity-50" />
        </div>
        <div class="bg-surface-container-low size-3 overflow-hidden rounded-sm">
          <div class="bg-primary size-full opacity-75" />
        </div>
        <div class="bg-surface-container-low size-3 overflow-hidden rounded-sm">
          <div class="bg-primary size-full opacity-100" />
        </div>
        <span>More</span>
      </div>
    </div> -->

    <div class="flex gap-3">
      <div class="flex h-full flex-col justify-evenly">
        <!-- <span class="label-small">{{ weekdays[0] }}</span> -->
        <span class="label-small">{{ weekdays[1] }}</span>
        <!-- <span class="label-small">{{ weekdays[2] }}</span> -->
        <span class="label-small">{{ weekdays[3] }}</span>
        <!-- <span class="label-small">{{ weekdays[4] }}</span> -->
        <span class="label-small">{{ weekdays[5] }}</span>
      </div>

      <div class="flex-1 overflow-x-auto">
        <div class="flex"></div>

        <div class="grid grid-flow-col grid-rows-7 gap-1" ref="container">
          <div v-for="_ in getDayOfWeek(graph[0]!.date, locale)" />

          <div
            v-for="(node, i) in graph"
            class="bg-surface-container-low size-3 overflow-hidden rounded-sm"
            :title="node.date.toString()"
            :key="i"
          >
            <div
              :style="{ opacity: node.activity / weightedMax }"
              class="bg-primary size-full"
            />
          </div>
        </div>
      </div>
    </div>
  </md-outlined-card>
</template>
