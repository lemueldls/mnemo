<script setup lang="ts">
import { redFromArgb } from "@material/material-color-utilities";

import { Rgb, ThemeColors, FileId } from "mnemo-wasm";

import type { Rgba } from "@material/material-color-utilities";

import type { EditorStateConfig } from "@codemirror/state";
import type { Package } from "~~/server/api/list-packages";

const dark = useDark();

const { d } = useI18n();

const spaceId = useRouteQuery("space");

const spaces = await useSpaces();
const space = computed(() => spaces.value[spaceId.value]!);

const dailyNoteRefs = await useStorageKeys(`spaces/${spaceId.value}/daily`);

const dailyNotes = await Promise.all(
  dailyNoteRefs.map(async (noteRef) => {
    const item = await useStorageItem(noteRef, "");
    return item.value;
  }),
);

const typstState = await useTypst();

const pixelPerPoint = ref(window.devicePixelRatio);
const pxToPt = (px: number) => px * window.devicePixelRatio * (72 / 96);

const { palette } = useMaterialTheme()!;

function parseColor(color: Rgba): Rgb {
  return new Rgb(color.r, color.g, color.b);
}

typstState.pt = pixelPerPoint.value;
typstState.size = 16 / pixelPerPoint.value;
typstState.theme = new ThemeColors(
  parseColor(palette.primary),
  parseColor(palette.secondary),
  parseColor(palette.tertiary),
  parseColor(palette.outline),
  parseColor(palette.onPrimaryContainer),
  parseColor(palette.onSecondaryContainer),
  parseColor(palette.onTertiaryContainer),
  parseColor(palette.onBackground),
);

const packages = await useStorageItem<Package[]>(
  `spaces/${spaceId.value}/packages.json`,
  [],
);
// watchImmediate(packages, async (packages) => {
//   await Promise.all(packages.map((pkg) => installTypstPackage(pkg)));
// });
await Promise.all(packages.value.map((pkg) => installTypstPackage(pkg)));

typstState.resize(200);

const path = `spaces/${spaceId.value}/render.typ`;
const fileId = typstState.insertFile(path, dailyNotes.join("\n"));
const pdfEncoded = typstState.renderPdf(fileId);

const pdfUrl = `data:application/pdf;base64,${pdfEncoded}`;
</script>

<template>
  <iframe :src="pdfUrl" class="w-full h-full" />
</template>

<style lang="scss">
/* #space-page {
  @apply absolute inset-0;
} */
</style>
