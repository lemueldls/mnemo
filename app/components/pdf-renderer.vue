<script setup lang="ts">
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

    return { note, item: item.value };
  }),
).then((notes) =>
  notes
    .filter(({ item }) => item)
    .map(({ note, item }) => {
      const time = decodeTime(note.id);
      const date = d(time, { weekday: "long", month: "long", day: "numeric" });

      return (
        `#set page(fill:rgb(0,0,0,0),width:730pt,height:auto,margin:16pt,header:none,footer:none)\n#align(right)[#text(size:14pt,fill:theme.on-primary-container,[${date}])]\n` +
        item
      );
    }),
);

const prelude = await useStorageItem(
  () => `spaces/${spaceId.value}/prelude/main.typ`,
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

const path = `spaces/${spaceId.value}/export.typ`;
const fileId = typstState.insertFile(path, dailyNotes.join("\n"));

const { bytes, diagnostics } = typstState.renderPdf(fileId);
const pdf = bytes ? new Uint8Array(bytes) : null;
</script>

<template>
  <div
    class="flex size-full items-center justify-center overflow-hidden px-4 pb-4"
  >
    <div v-if="diagnostics.length > 0" class="rounded-lg p-4">
      <pre
        v-for="(diagnostic, i) in diagnostics"
        :key="i"
        :class="{
          'bg-m3-error-container text-m3-on-error-container':
            diagnostic.severity === 'error',
          // 'bg-m3-warning-container text-m3-on-warning-container': diagnostic.severity === 'warning',
          // 'bg-m3-info-container text-m3-on-info-container': diagnostic.severity === 'info',
        }"
      ><strong>{{ diagnostic.severity }}</strong> {{ diagnostic.message }}</pre>
    </div>

    <md-outlined-card
      v-if="pdf"
      class="max-w-180 m-4 h-full w-full overflow-hidden"
    >
      <LazyEmbededPdf v-model="pdf" />
    </md-outlined-card>
  </div>
</template>
