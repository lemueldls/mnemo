<script setup lang="ts">
import {
  fromAbsolute,
  getLocalTimeZone,
  toCalendarDate,
} from "@internationalized/date";
import { createId } from "@paralleldrive/cuid2";

import type { StickyNote } from "~/composables/sticky";

definePageMeta({ layout: "empty" });

const { d } = useI18n();

const spaceId = usePageRouteQuery("id");
watchImmediate(spaceId, (spaceId) => {
  if (!spaceId) throw createError({ status: 404 });
});

const spaces = await useSpaces();
const space = reactiveComputed(() => spaces.value[spaceId.value!]!);

watchImmediate(
  space,
  (space) => {
    if (!space) throw createError({ status: 404 });
    spaces.set(spaceId.value, space);
  },
  { deep: true },
);

useHead({ title: () => space.name });

const { medium } = useBreakpoints(breakpointsM3);

const infoOpen = ref(false);
const settingsOpen = ref(false);

const preludeOpen = ref(false);
// const focusOpen = ref(false);
const stickyNotesOpen = ref(false);
const packagesOpen = ref(false);
const screenshotOpen = ref(false);

const router = useRouter();

function deleteSpace() {
  spaces.delete(spaceId.value);

  void router.push("/");
  infoOpen.value = false;
}

const screenshotBlob = ref<Blob>();
const screenshotUrl = ref<string>();

async function screenshot() {
  const { default: html2canvas } = await import("@html2canvas/html2canvas");

  screenshotUrl.value = "";
  screenshotBlob.value = undefined;

  screenshotOpen.value = true;

  const canvas = await html2canvas(document.querySelector("#editor")!, {
    backgroundColor: null,
    ignoreElements: (el) => el.id === "sidebar",
    // scale: window.devicePixelRatio,
    // imageTimeout:
  });

  canvas.toBlob((blob) => {
    screenshotBlob.value = blob!;
    screenshotUrl.value = URL.createObjectURL(blob!);
  });
}

function copyScreenshot() {
  window.navigator.clipboard.write([
    new ClipboardItem({ "image/png": screenshotBlob.value! }),
  ]);
}

const dailyNotes = ref<DailyNote[]>([]);

await watchImmediateAsync(spaceId, async (spaceId) => {
  const resolvedDailyNotes = await useDailyNotes(spaceId);
  const notes = await loadDailyNotes(spaceId, resolvedDailyNotes.value, false);

  dailyNotes.value = notes;
  resolvedDailyNotes.value = notes;
});

const notes = useArrayMap(dailyNotes, (note) => {
  const {
    id,
    datetime: [year, month, day, hour, minute],
  } = note;

  const date = Date.UTC(year, month, day, hour, minute);

  const textDate = d(date, {
    weekday: "long",
    month: "long",
    day: "numeric",
  });

  const timeZone = getLocalTimeZone();
  const dateTime = fromAbsolute(date, timeZone);
  const calendarDate = toCalendarDate(dateTime);

  return { id, calendarDate, textDate };
});

const currentNoteId = usePageRouteQuery("note");

const currentNote = computed(() => {
  const noteIndexById = currentNoteId.value
    ? notes.value.findLastIndex((note) => note.id === currentNoteId.value)
    : -1;

  const currentNoteIndex =
    noteIndexById === -1 ? notes.value.length - 1 : noteIndexById;

  return notes.value[currentNoteIndex]!;
});

watchImmediate(currentNote, (note) => {
  if (currentNoteId.value !== note.id) currentNoteId.value = note.id;
});

const deferredSpaceId = computedWithControl(currentNote, () => spaceId.value);

const nextDayId = computed(() => {
  const index = notes.value.findIndex(
    (note) => note.id === currentNote.value!.id,
  );

  return index === notes.value.length - 1
    ? undefined
    : notes.value[index + 1]!.id;
});
const previousDayId = computed(() => {
  const index = notes.value.findIndex(
    (note) => note.id === currentNote.value!.id,
  );

  return index === 0 ? undefined : notes.value[index - 1]!.id;
});

const currentDate = computed({
  get: () => currentNote.value.calendarDate,
  set(selectedDate) {
    const selectedNote = notes.value.findLast(
      (note) => note.calendarDate.compare(selectedDate) === 0,
    );

    if (selectedNote) currentNoteId.value = selectedNote.id;
  },
});
const datesWithNotes = useArrayMap(notes, (note) => note.calendarDate);

const preludePath = ref("main");

// const stickyNotes = ref(await listStickyNotes(spaceId.value));
const stickyNotes = await useStorageItem<{ [id: string]: StickyNote }>(
  () => `spaces/${spaceId.value}/sticky/notes.json`,
  {},
);
// stickyNotes.value = {};
const activeStickyNotes = ref<StickyNote[]>([]);

async function createStickyNote() {
  const datetime = d(Date.now(), {
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "numeric",
  });

  const id = createId();
  const note = {
    id,
    title: datetime,
    x: 128,
    y: 128,
    rx: 0.5,
    ry: 0.5,
    width: 384,
    height: 256,
  };

  stickyNotes.value[id] = note;
  activeStickyNotes.value.push(note);

  stickyNotesOpen.value = false;
}
</script>

<template>
  <mx-theme id="space-page" :color="space.color">
    <sticky-note
      v-for="(note, i) in activeStickyNotes"
      :key="note.id"
      v-model="activeStickyNotes[i]"
      :space-id="spaceId"
      @mousedown="
        () => {
          // const lastNote = activeStickyNotes.at(-1);
          // if (lastNote) {
          //   const currentNote = activeStickyNotes[i];
          //   // activeStickyNotes[i] = lastNote;
          //   // if (currentNote)
          //   //   activeStickyNotes[activeStickyNotes.length - 1] = currentNote;
          // }
        }
      "
      @close="
        activeStickyNotes = activeStickyNotes.filter(({ id }) => note.id !== id)
      "
    />

    <mx-page>
      <div class="flex h-full flex-1">
        <div class="flex size-full flex-1 flex-col">
          <mx-top-app-bar>
            <template #leading>
              <nuxt-link-locale to="/">
                <md-icon-button>
                  <md-icon>arrow_back</md-icon>
                </md-icon-button>
              </nuxt-link-locale>
            </template>

            <div class="flex flex-1 items-center justify-center gap-2">
              <md-icon v-if="space.icon">{{ space.icon }}</md-icon>

              <span class="line-clamp-1" :title="space.name">
                {{ space.name }}
              </span>
            </div>

            <template #trailing>
              <md-icon-button @click="infoOpen = true">
                <md-icon>info</md-icon>
              </md-icon-button>

              <md-icon-button @click="settingsOpen = true">
                <md-icon>settings</md-icon>
              </md-icon-button>
            </template>
          </mx-top-app-bar>

          <div
            class="medium:pr-0 flex min-h-0 flex-1 justify-center gap-4 pb-3 pl-3 pr-3"
          >
            <!-- <md-outlined-card class="p-0! h-full flex-1 overflow-hidden">
              <LazyEmbededPdf model-value="csc104.pdf" monochrome />
            </md-outlined-card> -->

            <div class="medium:ml-3 max-w-180 relative size-full">
              <div class="absolute left--6 pb-8 pt-16">
                <div id="sidebar" class="flex flex-col gap-4 overflow-auto">
                  <div class="sidebar-button" title="Prelude">
                    <div
                      class="sidebar-button__inner"
                      @click="preludeOpen = true"
                    >
                      <md-ripple />
                      <md-icon>code</md-icon>
                    </div>
                  </div>
                  <!-- <div
                    class="sidebar-button"
                    title="Timer"
                    @click="focusOpen = true"
                  >
                    <div class="sidebar-button__inner">
                      <md-ripple />
                      <md-icon>av_timer</md-icon>
                    </div>
                  </div> -->
                  <div
                    class="sidebar-button"
                    title="Sticky Notes"
                    @click="stickyNotesOpen = true"
                  >
                    <div class="sidebar-button__inner">
                      <md-ripple />
                      <md-icon>sticky_note_2</md-icon>
                    </div>
                  </div>
                  <div
                    class="sidebar-button"
                    title="Packages"
                    @click="packagesOpen = true"
                  >
                    <div class="sidebar-button__inner">
                      <md-ripple />
                      <md-icon>package_2</md-icon>
                    </div>
                  </div>
                  <div
                    class="sidebar-button"
                    title="Screenshot"
                    @click="screenshot"
                  >
                    <div class="sidebar-button__inner">
                      <md-ripple />
                      <md-icon>camera</md-icon>
                    </div>
                  </div>
                  <NuxtLinkLocale
                    class="sidebar-button"
                    title="Export"
                    :to="`/export?space=${spaceId}`"
                  >
                    <div class="sidebar-button__inner">
                      <md-ripple />
                      <md-icon>export_notes</md-icon>
                    </div>
                  </NuxtLinkLocale>
                </div>
              </div>

              <md-elevated-card id="editor">
                <div class="flex items-center justify-between p-2">
                  <md-divider class="w-2" />

                  <div class="z-2">
                    <mx-modal-date-picker
                      v-model:date="currentDate"
                      :marked-dates="datesWithNotes"
                      disable-unmarked-dates
                    >
                      <md-text-button class="font-mono">
                        {{ currentNote.textDate }}
                      </md-text-button>
                    </mx-modal-date-picker>
                  </div>

                  <md-divider class="flex-1" />

                  <md-icon-button
                    :disabled="!previousDayId"
                    @click="currentNoteId = previousDayId!"
                  >
                    <md-icon>keyboard_arrow_left</md-icon>
                  </md-icon-button>
                  <md-icon-button
                    :disabled="!nextDayId"
                    @click="currentNoteId = nextDayId!"
                  >
                    <md-icon>keyboard_arrow_right</md-icon>
                  </md-icon-button>
                </div>

                <editor
                  v-if="currentNote"
                  v-model="currentNote.id"
                  kind="daily"
                  :space-id="deferredSpaceId"
                  class="p-0"
                />
              </md-elevated-card>
            </div>
          </div>

          <side-bar v-if="!medium" direction="horizontal" />
        </div>
      </div>

      <md-dialog
        :open="infoOpen"
        class="w-full max-w-xl"
        @closed="infoOpen = false"
      >
        <span slot="headline" class="flex items-center gap-2">
          <md-icon>{{ space.icon }}</md-icon>

          {{ space.name }}
        </span>

        <form
          id="edit-space-form"
          slot="content"
          method="dialog"
          class="flex flex-col gap-8"
        >
          <edit-space v-model="space" />
        </form>

        <div slot="actions">
          <md-text-button
            class="text-error"
            form="edit-space-form"
            @click.prevent="deleteSpace"
          >
            Delete
          </md-text-button>
          <md-text-button form="edit-space-form">Confirm</md-text-button>
        </div>
      </md-dialog>

      <md-dialog
        :open="preludeOpen"
        class="w-full max-w-2xl"
        @closed="preludeOpen = false"
      >
        <span slot="headline" class="flex items-center justify-between">
          Prelude
        </span>

        <div slot="content" class="h-md">
          <editor v-model="preludePath" :space-id="spaceId" kind="prelude" />
        </div>
      </md-dialog>

      <md-dialog :open="stickyNotesOpen" @closed="stickyNotesOpen = false">
        <span slot="headline" class="flex items-center justify-between">
          Sticky Notes

          <md-icon-button @click="createStickyNote">
            <md-icon>add</md-icon>
          </md-icon-button>
        </span>

        <form slot="content" method="dialog">
          <div class="mb-4 flex items-center gap-4">
            <!-- <span class="flex-1 display-small">
              {{ useShortDate(currentNote) }}
            </span>

            <md-icon-button @click="nextDay">
              <md-icon>chevron_left</md-icon>
            </md-icon-button>
            <md-icon-button @click="previousDay">
              <md-icon>chevron_right</md-icon>
            </md-icon-button> -->
          </div>

          <div class="max-h-100 overflow-auto">
            <file-tree-item
              v-for="note in Object.values(stickyNotes).toReversed()"
              :key="note.id"
              :active="false"
              class="flex justify-between gap-8"
              @click="activeStickyNotes.push(note)"
            >
              <span class="flex flex-1 items-center gap-2">
                <md-icon>
                  {{ false ? "note_filled" : "note" }}
                </md-icon>

                {{ note.title }}
              </span>

              <md-icon-button>
                <md-icon>more_vert</md-icon>
              </md-icon-button>
            </file-tree-item>
          </div>
        </form>
      </md-dialog>

      <Packages v-model="packagesOpen" :space-id="spaceId" />

      <md-dialog :open="screenshotOpen" @closed="screenshotOpen = false">
        <span slot="headline">Screenshot</span>

        <div slot="content" class="flex items-center justify-center">
          <img
            v-if="screenshotUrl"
            :src="screenshotUrl"
            class="h-full max-h-full w-full max-w-full"
          />
          <md-progress-circular v-else indeterminate />
        </div>

        <div slot="actions">
          <md-text-button @click="copyScreenshot">Copy</md-text-button>
        </div>
      </md-dialog>

      <settings v-model="settingsOpen" />

      <new-task />
      <edit-task />

      <side-bar v-if="medium" direction="vertical" />
    </mx-page>
  </mx-theme>
</template>

<style lang="scss">
#space-page {
  @apply absolute inset-0;
}

#file-tree {
  @apply border-(outline r) w-64;
}

#editor {
  @apply h-full;
}

.sidebar-button {
  @apply pl-3.25 transition-all duration-200 hover:pl-0;
  // @apply medium:z-0 z-1 medium:hover:pl-0 pl-3 transition-all duration-200;

  &__inner {
    @apply bg-surface-container-high text-on-surface-variant relative flex h-12 w-6 cursor-pointer items-center justify-center;
  }
}
</style>
