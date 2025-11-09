<script setup lang="ts">
import { isToday, today, type CalendarDate } from "@internationalized/date";

const { d } = useI18n();

const timeZone = useTimeZone();
const activeDate = ref(today(timeZone)) as Ref<CalendarDate>;

const containerRef = useTemplateRef("container");
const caretRef = useTemplateRef("caret");

const scrollHeight = useScrollHeight(containerRef);
const newSpaceDialogOpen = ref(false);
const editSpaceDialogOpen = ref(false);

const { startWeekday, endWeekday, totalWeekdays } = useWeekdays();

onMounted(() => {
  const caret = caretRef.value!;

  useIntervalFn(
    () => {
      if (isToday(activeDate.value, timeZone)) {
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

const days = computed(() => {
  const start = startWeekday.value + 1;

  return Array.from({ length: totalWeekdays.value }).map((_, day) =>
    d(Date.UTC(0, 0, day + start), { weekday: "short" }),
  );
});

const newSpaceOpen = useNewSpaceOpen();

const newSpaceId = ref<string>();
const newSpaceDays = reactive(new Set<number>());
const newSpaceFrom = ref<string>();
const newSpaceTo = ref<string>();

function openDialog(day: number, hour: number) {
  newSpaceDays.clear();
  newSpaceDays.add(day);
  newSpaceFrom.value = hourToTime(hour - 1);
  newSpaceTo.value = hourToTime(hour);

  newSpaceDialogOpen.value = true;
}

const editScheduleDay = ref<number>();
const editScheduleIndex = ref<number>();
const editingScheduleRef = ref<ScheduleItem>();
const editingScheduleFrom = ref<string>();
const editingScheduleTo = ref<string>();

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
    const spaceId = newSpaceId.value!;
    if (!spaceId) throw createError("space required");

    schedule.value[day]!.push({
      spaceId,
      from: timeToMinutes(newSpaceFrom.value!),
      to: timeToMinutes(newSpaceTo.value!),
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
  <div class="calendar">
    <div class="calendar__head">
      <span class="w-12" />

      <div v-for="day in days" :key="day" class="calendar__cell">
        {{ day }}
      </div>
    </div>

    <div ref="container" class="calendar__body">
      <div class="w-12">
        <span
          v-for="hour in 24"
          :key="hour"
          class="label-medium flex h-12 items-start justify-end pr-2"
        >
          {{ $d(Date.UTC(0, 0, 0, hour - 20), { hour: "numeric" }) }}
        </span>
      </div>

      <div
        v-for="day in days.length"
        :key="day - 1 + startWeekday"
        class="calendar__body-column"
      >
        <div
          v-for="hour in 24"
          :key="hour"
          class="calendar__cell relative flex cursor-pointer items-center"
          @click="openDialog(day - 1 + startWeekday, hour)"
        >
          <md-ripple />

          <div class="border-(b outline-variant) b-b-dashed w-full" />
        </div>

        <template
          v-for="({ spaceId, from, to }, i) in schedule[day - 1 + startWeekday]"
          :key="i"
        >
          <mx-theme
            v-if="spaces![spaceId]"
            :color="spaces[spaceId].color"
            harmonize
            :style="{
              top: `${(from / 60) * (scrollHeight / 24)}px`,
              height: `${(to / 60 - from / 60) * (scrollHeight / 24)}px`,
            }"
            class="bg-primary-container bg-op-90 text-on-primary-container body-small absolute flex w-full cursor-pointer flex-col items-center justify-center rounded-xl p-2 text-center"
            @click="openEditDialog(day, i)"
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
          </mx-theme>
        </template>
      </div>

      <span ref="caret" class="border-(b error) absolute w-full select-none" />
    </div>
  </div>

  <md-dialog
    :open="newSpaceDialogOpen"
    @closed="newSpaceDialogOpen = false"
    @submit.prevent="createScheduleItem"
  >
    <span slot="headline">New Calendar Space</span>

    <form
      id="weekly-calendar-new-form"
      slot="content"
      class="flex flex-col gap-4 p-4"
      method="dialog"
    >
      <span class="label-large">Space</span>

      <div class="grid grid-cols-2 gap-4">
        <mx-theme
          v-for="(space, id) in spaces"
          :key="id"
          :color="space.color"
          harmonize
        >
          <md-outlined-card
            class="flex flex-col gap-2 p-3"
            @click="newSpaceId = id as string"
          >
            <md-ripple />

            <div class="flex flex-row items-center justify-between gap-2">
              <div class="h-6">
                <md-icon v-if="space.icon" class="text-primary">
                  {{ space.icon }}
                </md-icon>
              </div>

              <md-radio
                name="space"
                :value="id"
                :checked="id === newSpaceId"
                required
              />
            </div>
            <span class="title-large line-clamp-1 flex-1" :title="space.name">
              {{ space.name }}
            </span>
          </md-outlined-card>
        </mx-theme>

        <md-elevated-card
          v-if="Object.keys(spaces).length < 1"
          class="cursor-pointer p-4"
          @click="newSpaceOpen = true"
        >
          <md-ripple />

          <div class="flex flex-row items-center justify-between gap-2">
            <md-icon class="text-primary">add</md-icon>
          </div>
          <span class="title-large flex-1">Create a New Space</span>
        </md-elevated-card>
      </div>

      <span class="label-large">Day</span>
      <div class="flex gap-2">
        <md-filled-card v-for="(day, i) in days" :key="i" class="flex-1 p-4">
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
            <span class="label-large">{{ day }}</span>
          </label>
        </md-filled-card>
      </div>

      <span class="label-large">Time</span>
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
.calendar {
  @apply border-outline-variant flex h-full flex-col rounded-xl border;

  &__head {
    @apply border-outline-variant flex justify-between overflow-y-hidden border-b;

    scrollbar-gutter: stable;
  }

  &__body {
    @apply relative flex h-full overflow-y-scroll;

    scrollbar-gutter: stable;
  }

  &__cell {
    @apply border-outline-variant border-(b l) flex h-12 flex-1 items-center justify-center;
  }

  &__body &__cell {
    &:last-of-type {
      @apply border-b-none;
    }
  }

  &__body-column {
    @apply relative flex-1;
  }
}
</style>
