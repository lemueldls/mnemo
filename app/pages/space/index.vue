<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { Note } from "~/composables/spaces";

import html2canvas from "@html2canvas/html2canvas";

definePageMeta({ layout: "space" });

const { d } = useI18n();

const { query } = useRoute();
const spaceId = [query.id].flat()[0]!.toString();

const spaces = await listSpaces();
const space = spaces.value[spaceId]!;

console.log({ spaces, spaceId, space });

const { compact } = useBreakpoints(breakpointsM3);

const infoOpen = ref(false);

const preludeOpen = ref(false);
const focusOpen = ref(false);
const stickyNotesOpen = ref(false);
const packagesOpen = ref(false);
const screenshotOpen = ref(false);

const dark = useDark();

const screenshotBlob = ref<Blob>();
const screenshotUrl = ref<string>();

async function screenshot() {
  screenshotUrl.value = "";
  screenshotBlob.value = undefined;

  screenshotOpen.value = true;

  const canvas = await html2canvas(
    document.querySelector("#editor-container")!,
    {
      backgroundColor: null,
      ignoreElements: (el) => el.id === "sidebar",
      // scale: window.devicePixelRatio * 1,
      // imageTimeout:
    },
  );

  canvas.toBlob((blob) => {
    screenshotBlob.value = blob!;
    screenshotUrl.value = URL.createObjectURL(blob!);
  });
}

function copyScreenshot() {
  navigator.clipboard.write([
    new ClipboardItem({ "image/png": screenshotBlob.value! }),
  ]);
}

async function getDailyNotes() {
  const today = new Date();

  const notes = await useStorageItem<Note[]>(
    `spaces/${spaceId}/daily/notes.json`,
    [],
  );

  return computed(() => {
    let addToday = true;

    for (const note of notes.value) {
      const date = new Date(
        note.datetime[0],
        note.datetime[1],
        note.datetime[2],
      );
      if (
        date.getFullYear() === today.getFullYear() &&
        date.getMonth() === today.getMonth() &&
        date.getDate() === today.getDate()
      ) {
        addToday = false;
        break;
      }
    }

    // if (addToday) {
    //   const id = ulid();

    //   const date = new Date(decodeTime(id));
    //   const datetime: [number, number, number, number, number] = [date.getFullYear(), date.getMonth() , date.getDate(), date.getHours(), date.getMinutes()];

    //   notes.value.push({ id, datetime });
    // }

    return notes.value;
  });
}

const { data: notes } = await useAsyncData(
  "get_daily_notes",
  async () => {
    const notes = await invoke<Note[]>("get_daily_notes", {
      spaceId,
    });

    const item = await useStorageItem<Note[]>(
      `spaces/${spaceId}/daily/notes.json`,
      [],
    );
    item.value = notes;

    // const item = await useStorageItem<Note[]>(`spaces/${spaceId}/daily/notes.json`, []);
    // const notes = item.value!;

    // const notes = (await getDailyNotes()).value;

    return notes.map((note) => {
      const {
        id,
        datetime: [year, month, day, hour, minute],
      } = note;
      const date = d(new Date(Date.UTC(year, month - 1, day, hour, minute)), {
        weekday: "long",
        month: "long",
        day: "numeric",
      });

      return { id, date };
    });
  },
  { default: () => [] },
);

const currentNoteIndex = ref(0);
const currentNote = computed(() => notes.value[currentNoteIndex.value]);

const nextDayIndex = computed(() => {
  const index = notes.value.findIndex(
    (note) => note.id === currentNote.value!.id,
  );

  return index === 0 ? -1 : index - 1;
});
const previousDayIndex = computed(() => {
  const index = notes.value.findIndex(
    (note) => note.id === currentNote.value!.id,
  );

  return index === notes.value.length - 1 ? -1 : index + 1;
});

// const stickyNotes = ref(await listStickyNotes(spaceId));
const stickyNotes = ref<StickyNote[]>([]);
const activeStickyNotes = ref<StickyNote[]>([]);

async function loadStickyNotes() {
  stickyNotes.value = await listStickyNotes(spaceId);
}
async function deleteStickyNoteAndReload(id: string) {
  await deleteStickyNote(spaceId, id);
  await loadStickyNotes();
}

await loadStickyNotes();
whenever(stickyNotesOpen, loadStickyNotes);

async function addStickyNote() {
  await newStickyNote(spaceId);
  await loadStickyNotes();
}
</script>

<template>
  <m3-theme id="space-page" :color="space.color" :dark="dark">
    <sticky-note
      v-for="note in activeStickyNotes"
      :key="note.id"
      :space-id="spaceId"
      :note="note"
      @close="
        activeStickyNotes = activeStickyNotes.filter((n) => n.id !== note.id)
      "
    />

    <m3-page>
      <div class="h-full flex flex-1">
        <div class="h-full flex flex-1 flex-col">
          <m3-top-app-bar>
            <template #leading>
              <nuxt-link-locale to="/">
                <md-icon-button>
                  <md-icon>arrow_back</md-icon>
                </md-icon-button>
              </nuxt-link-locale>
            </template>

            <div class="flex flex-1 items-center justify-center gap-2">
              <md-icon>{{ space.icon }}</md-icon>

              {{ space.name }}
            </div>

            <template #trailing>
              <md-icon-button @click="infoOpen = true">
                <md-icon>info</md-icon>
              </md-icon-button>
            </template>
          </m3-top-app-bar>

          <div
            class="flex items-center justify-center gap-6 h-full w-full overflow-hidden self-center p-6"
            id="editor-container"
          >
            <!-- <m3-outlined-card class="flex-1 h-full p-0! overflow-hidden">
              <pdf-viewer />
            </m3-outlined-card> -->

            <div class="flex-1 relative h-full max-w-180 w-full">
              <div
                class="flex flex-col gap-4 absolute left--6 my-16 overflow-auto"
                id="sidebar"
              >
                <div class="sidebar-button">
                  <div
                    class="sidebar-button__inner"
                    @click="preludeOpen = true"
                  >
                    <md-ripple />
                    <md-icon>code</md-icon>
                  </div>
                </div>
                <div class="sidebar-button" @click="focusOpen = true">
                  <div class="sidebar-button__inner">
                    <md-ripple />
                    <md-icon>av_timer</md-icon>
                  </div>
                </div>
                <div class="sidebar-button" @click="stickyNotesOpen = true">
                  <div class="sidebar-button__inner">
                    <md-ripple />
                    <md-icon>sticky_note</md-icon>
                  </div>
                </div>
                <div class="sidebar-button" @click="packagesOpen = true">
                  <div class="sidebar-button__inner">
                    <md-ripple />
                    <md-icon>package_2</md-icon>
                  </div>
                </div>
                <div class="sidebar-button" @click="screenshot">
                  <div class="sidebar-button__inner">
                    <md-ripple />
                    <md-icon>camera</md-icon>
                  </div>
                </div>
              </div>

              <m3-elevated-card id="editor">
                <!-- <div id="editor-title">
                {{ currentNote.name.split(/\.\w+$/)[0] }}

                <md-icon-button @click="dialog = true">
                  <md-icon>note_stack</md-icon>
                </md-icon-button>
              </div> -->
                <div id="editor-title" class="items-center gap-2">
                  <div class="h-1px w-2 bg-m3-outline-variant" />

                  <span class="m3-label-large">
                    {{ currentNote?.date }}
                  </span>

                  <div class="h-1px flex-1 bg-m3-outline-variant" />

                  <md-icon-button
                    @click="currentNoteIndex = previousDayIndex"
                    :disabled="previousDayIndex === -1"
                  >
                    <md-icon>keyboard_arrow_up</md-icon>
                  </md-icon-button>
                  <md-icon-button
                    @click="currentNoteIndex = nextDayIndex"
                    :disabled="nextDayIndex === -1"
                  >
                    <md-icon>keyboard_arrow_down</md-icon>
                  </md-icon-button>
                </div>

                <editor
                  v-if="currentNote"
                  kind="daily"
                  :space-id="spaceId"
                  v-model="currentNote.id"
                  class="h-full flex-1"
                />
              </m3-elevated-card>
            </div>

            <side-bar direction="horizontal" v-if="compact" />
          </div>
        </div>
      </div>

      <md-dialog :open="infoOpen" @closed="infoOpen = false">
        <span slot="headline" class="flex items-center justify-between">
          {{ space.name }}
        </span>

        <span slot="content">
          <pre>
            <code>
              {{ spaceId }}
            </code>
          </pre>
        </span>
      </md-dialog>

      <md-dialog :open="stickyNotesOpen" @closed="stickyNotesOpen = false">
        <span slot="headline" class="flex items-center justify-between">
          Sticky Notes

          <md-icon-button @click="addStickyNote">
            <md-icon>add</md-icon>
          </md-icon-button>
        </span>

        <form slot="content" method="dialog">
          <div class="mb-4 flex items-center gap-4">
            <!-- <span class="flex-1 m3-display-small">
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
              v-for="note in stickyNotes.toReversed()"
              :key="note.id"
              :active="false"
              class="flex justify-between gap-8"
              @click="activeStickyNotes.push(note)"
            >
              <span class="flex flex-1 items-center gap-2">
                <md-icon>
                  {{ false ? "note_filled" : "note" }}
                </md-icon>

                {{ note.name }}
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
            class="w-full h-full max-w-full max-h-full"
          />
          <md-progress-circular v-else indeterminate />
        </div>

        <div slot="actions">
          <md-text-button @click="copyScreenshot">Copy</md-text-button>
        </div>
      </md-dialog>

      <side-bar direction="vertical" v-if="!compact" />
    </m3-page>
  </m3-theme>
</template>

<style lang="scss">
#space-page {
  @apply absolute inset-0;
}

#file-tree {
  @apply w-64 border-(m3-outline r);
}

#editor {
  @apply flex flex-col gap-3 h-full;
}

#editor-title {
  @apply flex justify-between text-m3-on-primary-container w-full m3-headline-large bg-transparent outline-none;

  /* font-family: "Iosevka Quasi Custom", sans-serif; */
  font-family: "Iosevka Book Web", sans-serif;
}

.sidebar-button {
  @apply transition-all duration-200 pl-3.25 hover:pl-0;

  &__inner {
    @apply relative h-12 w-6 flex items-center justify-center bg-m3-surface-container-high cursor-pointer text-m3-on-surface-variant;
  }
}
</style>
