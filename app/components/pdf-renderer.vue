<script setup lang="ts">
import { redFromArgb } from "@material/material-color-utilities";

import { Rgb, ThemeColors, FileId } from "mnemo-wasm";

import type { Rgba } from "@material/material-color-utilities";

import type { EditorStateConfig } from "@codemirror/state";
import type { Package } from "~~/server/api/list-packages";
import type { Note } from "~/composables/spaces";

const dark = useDark();

const { d } = useI18n();

const spaceId = useRouteQuery("space");

const spaces = await useSpaces();
const space = computed(() => spaces.value[spaceId.value]!);

const dailyNotesRef = await useStorageItem<Note[]>(
  `spaces/${spaceId.value}/daily/notes.json`,
  []
);

const dailyNotes = await Promise.all(
  dailyNotesRef.value.toReversed().map(async (note) => {
    const item = await useStorageItem<string>(
      `spaces/${spaceId.value}/daily/${note.id}.typ`,
      ""
    );

    return item.value;
  })
).then((notes) => notes.filter((note) => note));

const typstState = await useTypst();

const pixelPerPoint = ref(window.devicePixelRatio);

const { palette } = useMaterialTheme()!;

function parseColor(color: Rgba): Rgb {
  return new Rgb(color.r, color.g, color.b);
}

typstState.setPt(pixelPerPoint.value);
typstState.setSize(16 / pixelPerPoint.value);
typstState.setTheme(
  new ThemeColors(
    parseColor(palette.primary),
    parseColor(palette.secondary),
    parseColor(palette.tertiary),
    parseColor(palette.outline),
    parseColor(palette.onPrimaryContainer),
    parseColor(palette.onSecondaryContainer),
    parseColor(palette.onTertiaryContainer),
    parseColor(palette.onBackground)
  )
);

const packages = await useStorageItem<Package[]>(
  `spaces/${spaceId.value}/packages.json`,
  []
);
// watchImmediate(packages, async (packages) => {
//   await Promise.all(packages.map((pkg) => installTypstPackage(pkg)));
// });
await Promise.all(packages.value.map((pkg) => installTypstPackage(pkg)));

// typstState.resize(200);

// const path = `spaces/${spaceId.value}/render.typ`;
// const fileId = typstState.insertFile(path, dailyNotes.join("\n"));
// const html = typstState.renderHtml(fileId);
</script>

<template>
  <div>
    [[render]]
    <pre>
      <code>
        {{ dailyNotes }}
      </code>
    </pre>
    <!-- <div class="w-full h-full" v-html="html" /> -->
  </div>
</template>

<style lang="scss">
/* #space-page {
  @apply absolute inset-0;
} */
</style>
