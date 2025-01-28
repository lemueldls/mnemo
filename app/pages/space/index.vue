<script setup lang="ts">
import { ulid } from "ulid";

import html2canvas from "@html2canvas/html2canvas";

definePageMeta({ layout: "space" });

const { d } = useI18n();

// const { query } = useRoute();
// const spaceId = [query.id].flat()[0]!.toString();

const spaceId = useRouteQuery<string>("id");

// console.log({ spaceId: spaceId.value });

watchImmediate(spaceId, (spaceId) => {
  if (!spaceId) throw createError({ status: 404 });
});

const spaces = await useSpaces();
const space = computed(() => spaces.value[spaceId.value!]!);

// move
function updateSpace() {
  console.log(spaces.value[spaceId.value], space.value);
  spaces.value[spaceId.value] = space;
}

const { medium } = useBreakpoints(breakpointsM3);

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
  window.navigator.clipboard.write([
    new ClipboardItem({ "image/png": screenshotBlob.value! }),
  ]);
}

const spaceNotes = await useSpaceNotes(spaceId);

const { data: notes } = await useAsyncData(
  "get_daily_notes",
  async () => {
    // const notes = await invoke<Note[]>("get_daily_notes", {
    //   spaceId: spaceId.value,
    // });

    // const item = await useStorageItem<Note[]>(
    //   `spaces/${spaceId.value}/daily/notes.json`,
    //   [],
    // );
    // item.value = notes;

    // const item = await useStorageItem<Note[]>(`spaces/${spaceId}/daily/notes.json`, []);
    // const notes = item.value!;

    console.log({ spaceNotes: spaceNotes.value });

    const notes = await loadDailyNotes(spaceNotes);

    return notes.map((note) => {
      const {
        id,
        datetime: [year, month, day, hour, minute],
      } = note;
      const date = d(new Date(Date.UTC(year, month, day, hour, minute)), {
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
const stickyNotes = await useStorageItem<StickyNote[]>(
  `spaces/${spaceId}/sticky/notes.json`,
  [],
);
const activeStickyNotes = ref<StickyNote[]>([]);

async function createStickyNote() {
  stickyNotes.value.push({
    id: ulid(),
    name: "",
    x: 40,
    y: 40,
    width: 500,
    height: 500,
  });
}
</script>

<template>
  <m3-theme id="space-page" :color="space.color" :dark="dark">
    <sticky-note
      v-for="(note, i) in activeStickyNotes"
      :key="note.id"
      :space-id="spaceId"
      :note="note"
      @mousedown="
        () => {
          const lastNote = activeStickyNotes.at(-1);

          if (lastNote) {
            const currentNote = activeStickyNotes[i];

            // activeStickyNotes[i] = lastNote;

            // if (currentNote)
            //   activeStickyNotes[activeStickyNotes.length - 1] = currentNote;
          }
        }
      "
      @close="
        activeStickyNotes = activeStickyNotes.filter(({ id }) => note.id !== id)
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
            id="editor-container"
            class="flex items-center justify-center gap-3 h-full w-full overflow-hidden self-center pl-6 pb-3"
          >
            <!-- <m3-outlined-card class="flex-1 h-full p-0! overflow-hidden">
              <pdf-viewer />
            </m3-outlined-card> -->

            <div class="flex-1 relative h-full max-w-180 w-full">
              <div
                id="sidebar"
                class="flex flex-col gap-4 absolute left--6 my-16 overflow-auto"
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
                    :disabled="previousDayIndex === -1"
                    @click="currentNoteIndex = previousDayIndex"
                  >
                    <md-icon>keyboard_arrow_up</md-icon>
                  </md-icon-button>
                  <md-icon-button
                    :disabled="nextDayIndex === -1"
                    @click="currentNoteIndex = nextDayIndex"
                  >
                    <md-icon>keyboard_arrow_down</md-icon>
                  </md-icon-button>
                </div>

                <editor
                  v-if="currentNote"
                  v-model="currentNote.id"
                  kind="daily"
                  :space-id="spaceId"
                  class="h-full flex-1"
                />
              </m3-elevated-card>
            </div>

            <side-bar v-if="!medium" direction="horizontal" />
          </div>
        </div>
      </div>

      <md-dialog :open="infoOpen" @closed="infoOpen = false">
        <span slot="headline" class="flex items-center justify-between">
          {{ space.name }}
        </span>

        <!-- <span slot="content">
          <pre>
                <code>
                {{ spaceId }}
                </code>
            </pre>
        </span> -->

        <form slot="content" method="dialog" class="flex flex-col gap-8">
          <edit-space v-model="space">
            <template #actions>
              <md-text-button @click="updateSpace">Confirm</md-text-button>
            </template>
          </edit-space>
        </form>
      </md-dialog>

      <md-dialog :open="preludeOpen" @closed="preludeOpen = false">
        <!-- <editor
          :space-id="spaceId"
          v-model="currentNote.id"
          kind="prelude"
          class="h-full flex-1"
        /> -->
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

      <side-bar v-if="medium" direction="vertical" />
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
