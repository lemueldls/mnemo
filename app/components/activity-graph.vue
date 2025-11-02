<script setup lang="ts">
import { getDayOfWeek, parseDate } from "@internationalized/date";

const showWeekends = false;
const startWeekday = showWeekends ? 0 : 1;
const endWeekday = showWeekends ? 7 : 6;
const totalWeekdays = endWeekday - startWeekday;

const container = useTemplateRef("container");
const { width } = useElementSize(container);

const scrollWidth = useScrollWidth(container);
const { x: scrollX } = useScroll(container);

// const cellSize = 12;
// const gapSize = 4;

const activityGraph = await useActivity();

const weightedMax = ref(12);

const { d, locale } = useI18n();

const leftFade = ref(0);
const rightFade = ref(0);
const maxFade = 32;

watchImmediate(
  [scrollWidth, scrollX, width],
  ([scrollWidth, scrollX, width]) => {
    if (!container.value) return;

    leftFade.value = Math.min(scrollX, maxFade);
    rightFade.value = Math.min(scrollWidth - scrollX - width, maxFade);
  },
);

watchImmediate([scrollWidth, width], ([scrollWidth]) => {
  scrollX.value = scrollWidth;
});

// const months = computed(() =>
//   Array.from({ length: 12 }).map((_, month) =>
//     d(Date.UTC(0, month + 1), { month: "long" }),
//   ),
// );

const weekdays = computed(() =>
  Array.from({ length: 7 }).map((_, weekday) =>
    d(Date.UTC(0, 0, weekday + 1), { weekday: "short" }),
  ),
);
</script>

<template>
  <div class="flex flex-col gap-3 overflow-hidden">
    <div class="flex items-stretch justify-between gap-3">
      <div class="flex flex-col">
        <div class="label-small flex-1" v-for="i in totalWeekdays">
          <span v-if="i % 2">{{ weekdays[startWeekday + i - 1] }}</span>
        </div>
      </div>

      <div class="activity-graph" ref="container" v-if="activityGraph[0]">
        <template
          v-for="i in getDayOfWeek(parseDate(activityGraph[0].date!), locale)"
        >
          <div v-if="i >= startWeekday" />
        </template>

        <template v-for="(node, i) in activityGraph" :key="i">
          <div
            class="bg-surface-container-low size-3 overflow-hidden rounded-sm"
            :title="node.date.toString()"
            v-if="i % endWeekday >= startWeekday"
          >
            <div
              :style="{ opacity: node.activity / weightedMax }"
              class="bg-primary size-full"
            />
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<style scoped>
.activity-graph {
  @apply grid flex-1 grid-flow-col gap-1 overflow-x-hidden;

  grid-template-rows: repeat(v-bind(totalWeekdays), minmax(0, 1fr));

  mask-image: linear-gradient(
    to right,
    transparent 0%,
    rgba(0, 0, 0, 0.25) calc(v-bind(leftFade) / 2 * 1px),
    black calc(v-bind(leftFade) * 1px),
    black calc(100% - v-bind(rightFade) * 1px),
    rgba(0, 0, 0, 0.25) calc(100% - v-bind(rightFade) / 2 * 1px),
    transparent 100%
  );
}
</style>
