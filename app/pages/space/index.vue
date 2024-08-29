<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { Note } from "~/composables/spaces";

definePageMeta({
  layout: "space",
  pageTransition: { name: "conjure" },
  // middleware({ query }) {
  //   let containsSpace = spaces.value!.some(([id, _]) => id === query.id);

  //   console.log({ query });

  //   if (!query.id || !containsSpace) return navigateTo("/");
  // },
});

const { d } = useI18n();

const { query } = useRoute();
const spaceId = [query.id].flat()[0]!.toString();

const spaces = await listSpaces();
const space = spaces.value!.find(([id, _]) => id === spaceId)![1];

const { smallerOrEqual } = useBreakpoints(breakpointsM3);
const mobile = smallerOrEqual("medium");

const infoOpen = ref(false);

const preludeOpen = ref(false);
const focusOpen = ref(false);
const stickyNotesOpen = ref(false);
const packagesOpen = ref(false);

// const space = [spaceId]!;
const dark = useDark();

const { data: notes } = await useAsyncData(
  "get_daily_notes",
  async () => {
    const notes = await invoke<Note[]>("get_daily_notes", {
      spaceId,
    });

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

watchEffect(() => {
  console.log(notes.value);
  // console.log(
  //   currentNoteIndex.value,
  //   nextDayIndex.value,
  //   previousDayIndex.value,
  // );
  // console.log(currentNote.value, notes.value[currentNoteIndex.value]);
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

          <div class="flex items-center justify-center gap-6 h-full w-full overflow-hidden self-center p-6">
            <sticky-note v-for="note in activeStickyNotes" :key="note.id" :space-id="spaceId" :note="note" @close="
                activeStickyNotes = activeStickyNotes.filter(
                  (n) => n.id !== note.id,
                )
              " />

            <!-- <m3-outlined-card class="flex-1 h-full p-0! overflow-hidden">
              <pdf-viewer />
            </m3-outlined-card> -->

            <div class="flex-1 relative h-full max-w-180 w-full">
              <div class="flex flex-col gap-4 absolute left--6 top-16">
                <div class="sidebar-button">
                  <div class="sidebar-button__inner" @click="preludeOpen = true">
                    <md-icon>code</md-icon>
                  </div>
                </div>
                <div class="sidebar-button" @click="focusOpen = true">
                  <div class="sidebar-button__inner">
                    <md-icon>av_timer</md-icon>
                  </div>
                </div>
                <div class="sidebar-button" @click="stickyNotesOpen = true">
                  <div class="sidebar-button__inner">
                    <md-icon>sticky_note</md-icon>
                  </div>
                </div>
                <div class="sidebar-button" @click="packagesOpen = true">
                  <div class="sidebar-button__inner">
                    <md-icon>package_2</md-icon>
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

                  <md-icon-button @click="currentNoteIndex = nextDayIndex" :disabled="nextDayIndex === -1">
                    <md-icon>keyboard_arrow_up</md-icon>
                  </md-icon-button>
                  <md-icon-button @click="currentNoteIndex = previousDayIndex" :disabled="previousDayIndex === -1">
                    <md-icon>keyboard_arrow_down</md-icon>
                  </md-icon-button>
                </div>

                <editor v-if="currentNote" kind="daily" :space-id="spaceId" v-model="currentNote.id"
                  class="h-full flex-1" />
              </m3-elevated-card>
            </div>

            <side-bar direction="horizontal" v-if="mobile" />
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
            <file-tree-item v-for="note in stickyNotes.toReversed()" :key="note.id" :active="false"
              class="flex justify-between gap-8" @click="activeStickyNotes.push(note)">
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

      <Packages v-model="packagesOpen" />

      <side-bar direction="vertical" v-if="!mobile" />
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
  @apply flex flex-col gap-4 h-full;
}

#editor-title {
  @apply flex justify-between text-m3-on-primary-container w-full m3-headline-large bg-transparent outline-none;

  /* font-family: "Iosevka Quasi Custom", sans-serif; */
  font-family: "Iosevka Book Web", sans-serif;
}

.sidebar-button {
  @apply transition-all duration-200 pl-4 hover:pl-0;

  &__inner {
    @apply h-12 w-6 flex items-center justify-center bg-m3-surface-variant cursor-pointer text-m3-on-surface-variant;
  }
}
</style>
