<script setup lang="ts">
import { VuePDF, usePDF } from "@tato30/vue-pdf";

import { Rgb, ThemeColors } from "mnemo-wasm";

import type { Rgba } from "@material/material-color-utilities";
import { decodeTime } from "ulid";

import type { Note } from "~/composables/spaces";

const spaceId = usePageRouteQuery("space");

const { d } = useI18n();

// const spaces = await useSpaces();
// const space = computed(() => spaces.value[spaceId.value]!);

const dailyNotesRef = await useStorageItem<Note[]>(
  `spaces/${spaceId.value}/daily/notes.json`,
  [],
);

const dailyNotes = await Promise.all(
  dailyNotesRef.value.toReversed().map(async (note) => {
    const item = await useStorageItem<string>(
      `spaces/${spaceId.value}/daily/${note.id}.typ`,
      "",
    );

    const time = decodeTime(note.id);
    const date = d(time, { weekday: "long", month: "long", day: "numeric" });

    return (
      `
        #set page(fill:rgb(0,0,0,0),width:730pt,height:auto,margin:16pt,header:none,footer:none)

        #align(right)[===== ${date}]
      ` + item.value
    );
  }),
).then((notes) => notes.filter((note) => note));

// dailyNotes.unshift(`
//   #set page(fill:rgb(0,0,0,0),width:730pt,margin:16pt)
// `);

const prelude = await useRefStorageItem(
  computed(() => `spaces/${spaceId.value}/prelude/main.typ`),
  "",
);

dailyNotes.unshift(prelude.value);

const typstState = await useTypst();

const pixelPerPoint = ref(window.devicePixelRatio);

const theme = useMaterialTheme()!;
const palette = computed(() => theme.value.palette);

function parseColor(color: Rgba): Rgb {
  return new Rgb(color.r, color.g, color.b);
}

watchImmediate([pixelPerPoint, palette], async ([pixelPerPoint, palette]) => {
  const typstState = await useTypst();

  typstState.setPt(pixelPerPoint);
  typstState.setSize(16);
  typstState.setTheme(
    new ThemeColors(
      parseColor(palette.primary),
      parseColor(palette.secondary),
      parseColor(palette.tertiary),
      parseColor(palette.outline),
      parseColor(palette.onPrimaryContainer),
      parseColor(palette.onSecondaryContainer),
      parseColor(palette.onTertiaryContainer),
      parseColor(palette.onBackground),
    ),
  );
});

const packages = await useInstalledPackages(spaceId.value);
// TODO: check if spamming
await Promise.all(
  packages.value
    .filter((pkg) => pkg.name !== "suiji")
    .map((pkg) => installTypstPackage(pkg)),
);

// typstState.resize(200);

const path = `spaces/${spaceId.value}/render.typ`;
const fileId = typstState.insertFile(path, dailyNotes.join("\n"));
// const html = typstState.renderHtml(fileId);

const bytes = typstState.renderPdf(fileId);
const { pdf, pages, info } = usePDF(bytes);
</script>

<template>
  <div class="flex size-full items-center justify-center">
    <!-- [[render]]
    <pre>
      <code>
        {{ dailyNotes }}
      </code>
    </pre> -->
    <!-- <div class="h-full w-full" v-html="html" /> -->

    <md-outlined-card class="m-4 h-full overflow-scroll">
      <VuePDF v-for="page in pages" :key="page" :pdf :page text-layer />
    </md-outlined-card>
  </div>
</template>

<style lang="scss">
@import "@tato30/vue-pdf/style.css";

/* #space-page {
  @apply absolute inset-0;
} */
</style>
