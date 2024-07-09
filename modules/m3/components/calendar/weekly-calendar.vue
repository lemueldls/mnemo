<script setup lang="ts">
const { d } = useI18n();

const activeDay = ref(new Date());

const container = ref<HTMLDivElement>();
const caret = ref<HTMLDivElement>();

const scrollHeight = ref(0);

onMounted(() => {
  scrollHeight.value = container.value!.scrollHeight;

  useIntervalFn(
    () => {
      if (isToday(activeDay.value)) {
        const now = new Date();

        caret.value!.style.top = `calc(${
          ((now.getHours() + now.getMinutes() / 60) / 24) * scrollHeight.value
        }px + 1.25rem)`;
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

const days = [1, 2, 3, 4, 5].map((day) =>
  d(Date.UTC(0, 0, day + 1), { weekday: "short" }),
);

const dark = useDark();

const spaces = listSpaces();
const { currentEvents } = useEvents();
</script>

<template>
  <div class="m3-calendar">
    <div class="m3-calendar__head">
      <span class="w-16" />

      <div v-for="day in days" :key="day" class="m3-calendar__cell">
        {{ day }}
      </div>
    </div>

    <div ref="container" class="m3-calendar__body">
      <div class="w-16">
        <span
          v-for="hour in 24"
          :key="hour"
          class="h-12 flex items-center justify-end pr-2"
        >
          {{ $d(Date.UTC(0, 0, 0, hour - 20), { hour: "numeric" }) }}
        </span>
      </div>

      <div
        v-for="day in days.length"
        :key="day"
        class="m3-calendar__body-column"
      >
        <div
          v-for="hour in 24"
          :key="hour"
          class="m3-calendar__cell flex items-center"
        >
          <div class="w-full border-(b m3-outline-variant)" />
        </div>

        <m3-theme
          v-for="({ space, from, to }, i) in currentEvents[day]"
          :key="i"
          :color="spaces[space].color"
          :dark="dark"
          harmonize
          :style="{
            top: `calc(${
              ((from.getHours() + from.getMinutes() / 60) / 24) * scrollHeight
            }px + 1.5rem)`,
            height: `${
              (to.getHours() +
                to.getMinutes() / 60 -
                (from.getHours() + from.getMinutes() / 60)) *
              (scrollHeight / 24)
            }px`,
          }"
          class="absolute w-full flex flex-col items-center justify-center rounded-xl bg-m3-primary-container bg-op-90 text-center text-m3-on-primary-container m3-body-small"
        >
          <span class="w-full truncate">
            {{ space }}
          </span>

          <span class="w-full truncate">
            {{ $d(from, { hour: "numeric", minute: "numeric" }) }} -
            {{ $d(to, { hour: "numeric", minute: "numeric" }) }}
          </span>
        </m3-theme>
      </div>

      <span ref="caret" class="absolute w-full border-(b m3-outline)" />
    </div>
  </div>
</template>

<style lang="scss">
.m3-calendar {
  @apply flex flex-col h-full;

  &__head {
    @apply flex justify-between;
  }

  &__body {
    @apply overflow-auto flex h-full relative;
  }

  &__cell {
    @apply h-12 flex items-center justify-center flex-1 border-m3-surface-variant border-(b l);
  }

  &__body &__cell {
    @apply b-b-dashed;
  }

  &__head &__cell {
    @apply border-t;

    &:first-of-type {
      @apply border-l rounded-tl-xl;
    }

    &:last-of-type {
      @apply rounded-tr-xl;
    }
  }

  &__body-column {
    @apply relative flex-1;
  }

  &__body-column:last-child &__cell {
    @apply border-r;
  }
}
</style>
