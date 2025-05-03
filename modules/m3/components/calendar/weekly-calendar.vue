<script setup lang="ts">
const { d } = useI18n();

const activeDay = ref(new Date());

const containerRef = useTemplateRef("container");
const caretRef = useTemplateRef("caret");

const scrollHeight = ref(0);
const newSpaceDialogOpen = ref(false);
const editSpaceDialogOpen = ref(false);

onMounted(() => {
  const container = containerRef.value!;
  const caret = caretRef.value!;

  scrollHeight.value = container.scrollHeight;

  useIntervalFn(
    () => {
      if (isToday(activeDay.value)) {
        const now = new Date();

        caret.style.top = `${
          ((now.getHours() + now.getMinutes() / 60) / 24) * scrollHeight.value
        }px`;
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

const newSpaceId = ref();
const newSpaceDays = reactive(new Set<number>());
const newSpaceFrom = ref();
const newSpaceTo = ref();

function openDialog(day: number, hour: number) {
  newSpaceDays.clear();
  newSpaceDays.add(day);
  newSpaceFrom.value = hourToTime(hour - 1);
  newSpaceTo.value = hourToTime(hour);

  newSpaceDialogOpen.value = true;
}

const editScheduleDay = ref();
const editScheduleIndex = ref();
const editingScheduleRef = ref<ScheduleItem>();
const editingScheduleFrom = ref();
const editingScheduleTo = ref();

function openEditDialog(day: number, index: number) {
  editScheduleDay.value = day;
  editScheduleIndex.value = index;

  const scheduleItem = schedule.value[day]![index]!;
  editingScheduleRef.value = scheduleItem;
  editingScheduleFrom.value = minutesToTime(scheduleItem.from);
  editingScheduleTo.value = minutesToTime(scheduleItem.to);

  editSpaceDialogOpen.value = true;
}

function hourToTime(hour: number) {
  return `${(hour % 24).toString().padStart(2, "0")}:00`;
}

function minutesToTime(minutes: number) {
  return `${Math.floor(minutes / 60)
    .toString()
    .padStart(2, "0")}:${(minutes % 60).toString().padStart(2, "0")}`;
}

const spaces = await useSpaces();
const schedule = await useSchedule();

// console.log({ schedule: schedule.value });

function createScheduleItem(event: SubmitEvent) {
  // const formData = new FormData(event.target as HTMLFormElement);

  // const spaceId = formData.get("space") as string;
  // const from = formData.get("from") as string;
  // const to = formData.get("to") as string;
  // const day = formData.get("day") as string;

  // schedule.value[Number.parseInt(day)]!.push({
  //   spaceId,
  //   from: timeToMinutes(from),
  //   to: timeToMinutes(to),
  // });

  for (const day of newSpaceDays) {
    const spaceId = newSpaceId.value;
    if (!spaceId) throw createError("space required");

    schedule.value[day]!.push({
      spaceId,
      from: timeToMinutes(newSpaceFrom.value),
      to: timeToMinutes(newSpaceTo.value),
    });
  }
}

function editScheduleItem(event: SubmitEvent) {
  const formData = new FormData(event.target as HTMLFormElement);

  const from = formData.get("from") as string;
  const to = formData.get("to") as string;

  const day = editScheduleDay.value!;
  const index = editScheduleIndex.value!;

  schedule.value[day]![index] = {
    spaceId: editingScheduleRef.value!.spaceId,
    from: timeToMinutes(from),
    to: timeToMinutes(to),
  };

  editSpaceDialogOpen.value = false;
}

function deleteScheduleItem() {
  const day = editScheduleDay.value!;
  const index = editScheduleIndex.value!;
  schedule.value[day]!.splice(index, 1);

  editSpaceDialogOpen.value = false;
}

function timeToMinutes(time: string) {
  const [hour, minute] = time.split(":");

  return Number.parseInt(hour!) * 60 + Number.parseInt(minute!);
}
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
          class="m3-label-medium flex h-12 items-start justify-end pr-2"
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
          class="m3-calendar__cell relative flex cursor-pointer items-center"
          @click="openDialog(day, hour)"
        >
          <md-ripple />

          <div class="border-(b m3-outline-variant) b-b-dashed w-full" />
        </div>

        <m3-theme
          v-for="({ spaceId, from, to }, i) in schedule[day]"
          :key="i"
          :color="spaces![spaceId]!.color"
          :dark="dark"
          harmonize
          :style="{
            top: `${(from / 60) * (scrollHeight / 24)}px`,
            height: `${(to / 60 - from / 60) * (scrollHeight / 24)}px`,
          }"
          class="bg-m3-primary-container bg-op-90 text-m3-on-primary-container m3-body-small absolute flex w-full cursor-pointer flex-col items-center justify-center rounded-xl p-2 text-center"
          @click="openEditDialog(day, i)"
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
        </m3-theme>
      </div>

      <span ref="caret" class="border-(b m3-outline) absolute w-full" />
    </div>
  </div>

  <md-dialog
    :open="newSpaceDialogOpen"
    @closed="newSpaceDialogOpen = false"
    @submit.prevent="createScheduleItem"
  >
    <span slot="headline">New Space</span>

    <form
      id="weekly-calendar-new-form"
      slot="content"
      class="flex flex-col gap-4 p-4"
      method="dialog"
    >
      <span class="m3-label-large">Space</span>

      <div class="grid grid-cols-2 gap-4">
        <m3-theme
          v-for="(space, id) in spaces"
          :key="id"
          :color="space.color"
          :dark="dark"
          harmonize
        >
          <label>
            <m3-elevated-card>
              <md-ripple />

              <div class="flex flex-row items-center justify-between gap-2">
                <md-icon class="text-m3-primary">{{ space.icon }}</md-icon>
                <md-radio
                  name="space"
                  :value="id"
                  :checked="id === newSpaceId"
                  touch-target="wrapper"
                  required
                  @change="newSpaceId = id"
                />
              </div>
              <span class="m3-title-medium flex-1">{{ space.name }}</span>
            </m3-elevated-card>
          </label>
        </m3-theme>
      </div>

      <span class="m3-label-large">Day</span>
      <div class="flex gap-2">
        <m3-filled-card v-for="(day, i) in days" :key="i" class="flex-1">
          <md-ripple />

          <label class="flex gap-4">
            <md-checkbox
              name="day"
              :value="i + 1"
              :checked="newSpaceDays.has(i + 1)"
              @change="
                newSpaceDays.has(i + 1)
                  ? newSpaceDays.delete(i + 1)
                  : newSpaceDays.add(i + 1)
              "
            />
            <span class="m3-label-large">{{ day }}</span>
          </label>
        </m3-filled-card>
      </div>

      <span class="m3-label-large">Time</span>
      <div class="flex gap-2">
        <md-outlined-text-field
          class="flex-1"
          label="From"
          name="from"
          type="time"
          :value="newSpaceFrom"
          required
          @input="newSpaceFrom = $event.target.value"
        />
        <md-outlined-text-field
          class="flex-1"
          label="To"
          name="to"
          type="time"
          :value="newSpaceTo"
          required
          @input="newSpaceTo = $event.target.value"
        />
      </div>
    </form>

    <div slot="actions" class="mt-4 flex gap-2">
      <div class="flex-[2]" />
      <md-filled-button form="weekly-calendar-new-form" class="flex-[3]">
        Create
      </md-filled-button>
    </div>
  </md-dialog>

  <md-dialog :open="editSpaceDialogOpen" @closed="editSpaceDialogOpen = false">
    <span slot="headline">Edit Space</span>

    <form
      id="weekly-calendar-edit-form"
      slot="content"
      class="min-w-lg flex flex-col gap-4 p-4"
      method="dialog"
      @submit.prevent="editScheduleItem"
    >
      <div class="flex gap-4">
        <md-outlined-text-field
          class="flex-1"
          label="From"
          name="from"
          type="time"
          :value="editingScheduleFrom"
          required
          @input="editingScheduleFrom = $event.target.value"
        />
        <md-outlined-text-field
          class="flex-1"
          label="To"
          name="to"
          type="time"
          :value="editingScheduleTo"
          required
          @input="editingScheduleTo = $event.target.value"
        />
      </div>
    </form>

    <div slot="actions" class="mt-4 flex gap-2">
      <md-filled-tonal-button
        form="weekly-calendar-edit-form"
        class="flex-[2]"
        @click.prevent="deleteScheduleItem"
      >
        Delete
      </md-filled-tonal-button>
      <md-filled-button form="weekly-calendar-edit-form" class="flex-[3]">
        Save
      </md-filled-button>
    </div>
  </md-dialog>
</template>

<style lang="scss">
.m3-calendar {
  @apply border-m3-outline-variant flex h-full flex-col rounded-tr-xl border-r;

  &__head {
    @apply flex justify-between;
  }

  &__body {
    @apply relative flex h-full overflow-y-auto;
  }

  &__cell {
    @apply border-m3-outline-variant border-(b l) flex h-12 flex-1 items-center justify-center;
  }

  // &__body &__cell {
  //   @apply b-b-dashed;
  // }

  &__head &__cell {
    @apply border-t;

    &:first-of-type {
      @apply rounded-tl-xl border-l;
    }

    &:last-of-type {
      @apply rounded-tr-xl;
    }
  }

  &__body-column {
    @apply relative flex-1;
  }
}
</style>
