<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

definePageMeta({
  layout: "space",
  pageTransition: { name: "conjure" },
  middleware ({ query }) {
    const spaces = listSpaces();

    if (!query.id || !spaces.value[query.id as string]) return navigateTo("/");
  },
});

const { d } = useI18n();

const { query } = useRoute();
const name = [query.id].flat()[0]!.toString();

const dialog = ref(false);

const space = listSpaces().value[name];
const dark = useDark();

const { data: files } = useAsyncData(
  "read_dir",
  async () => {
    const directory = await invoke<[number, string][]>("read_dir", { space: name });



    return directory.map(([date, name]) => {
      return { date: new Date(date), name };
    });
  },
  { server: false, default: () => [] },
);

function getOrCreateFile (date: Date) {
  const file = files.value.find((file) => file.date.getTime() === date.getTime());

  if (file) return file;

  return { date, name: `${useLongDate(date)}.typ` };
}

const activeFile = ref(getOrCreateFile(new Date()));


function nextDay () {
  // activeFile.value = getOrCreateFile(new Date(activeFile.value.date.getTime() + 86400000));
  const { date } = activeFile.value;
  activeFile.value = getOrCreateFile(new Date(
    date.getFullYear(),
    date.getMonth(),
    date.getDate() - 1,
  ));
}

function previousDay () {
  // activeFile.value = getOrCreateFile(new Date(activeFile.value.date.getTime() - 86400000));
  const { date } = activeFile.value;
  activeFile.value = getOrCreateFile(new Date(
    date.getFullYear(),
    date.getMonth(),
    date.getDate() + 1,
  ));
}
</script>

<template>
  <m3-theme id="space-page" :color="space?.color" :dark="dark">
    <m3-page>
      <div class="h-full flex flex-1 overflow-hidden">
        <div class="h-full flex flex-1 flex-col overflow-hidden">
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

              {{ name }}
            </div>

            <template #trailing>
              <md-icon-button>
                <md-icon>info</md-icon>
              </md-icon-button>
            </template>
          </m3-top-app-bar>

          <div class="relative h-full max-w-180 w-full self-center overflow-hidden p-6">
            <div class="absolute left-0 top-16 px-4 transition-all duration-200 hover:p-0">
              <div class="h-12 w-6 flex items-center justify-center bg-m3-surface-variant text-m3-on-surface-variant">
                <md-icon>bookmark</md-icon>
              </div>
            </div>

            <m3-elevated-card id="editor">
              <div id="editor-title">
                {{ activeFile.name.split(/\.\w+$/)[0] }}

                <md-icon-button @click="dialog = true">
                  <md-icon>note_stack</md-icon>
                </md-icon-button>
              </div>

              <editor :space="name" v-model="activeFile.name" class="h-full flex-1" />
            </m3-elevated-card>
          </div>
        </div>
      </div>

      <md-dialog :open="dialog" @closed="dialog = false">
        <span slot="headline">Notes</span>
        <form slot="content" method="dialog">
          <div class="mb-4 flex items-center gap-4">
            <span class="flex-1 m3-display-small">
              {{ useShortDate(activeFile.date) }}
            </span>

            <md-icon-button @click="nextDay">
              <md-icon>chevron_left</md-icon>
            </md-icon-button>
            <md-icon-button @click="previousDay">
              <md-icon>chevron_right</md-icon>
            </md-icon-button>
          </div>

          <div class="max-h-100 overflow-auto">
            <file-tree-item v-for="file in files.toReversed()" :key="file.name" :active="file.name === activeFile.name"
              class="flex justify-between gap-8" @click="activeFile = file">
              <span class="flex flex-1 items-center gap-2">
                <md-icon>
                  {{ file.name === activeFile.name ? "note_filled" : "note" }}
                </md-icon>

                {{ file.name }}
              </span>

              <span class="m3-label-medium">
                {{ useShortDate(file.date) }}
              </span>
            </file-tree-item>
          </div>
        </form>
      </md-dialog>

      <side-bar />
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

  font-family: "Iosevka Quasi Custom", sans-serif;
}
</style>
