<script setup lang="ts">
import { Rgb, ThemeColors } from "mnemo-wasm";
import { decodeTime } from "ulid";

import type { DailyNote } from "~/composables/notes";
import type { Rgba } from "~~/modules/mx/types";

const spaceId = usePageRouteQuery("space");

const { d } = useSharedI18n();

const spaces = await useSpaces();
const space = computed(() => spaces.value[spaceId.value]!);

const dailyNotesItem = await getStorageItem<DailyNote[]>(
  `spaces/${spaceId.value}/daily/notes.json`,
);

const dailyNotes = await Promise.all(
  dailyNotesItem!.map(async (note) => {
    const item = await getStorageItem<string>(`spaces/${spaceId.value}/daily/${note.id}.typ`);

    return { note, item };
  }),
).then((notes) =>
  notes
    .filter(({ item }) => item)
    .map(({ note, item }) => {
      const time = decodeTime(note.id);
      const date = d(time, { weekday: "long", month: "long", day: "numeric" });

      return (
        `#align(right)[#text(size:14pt,fill:theme.on-primary-container,[${date}])]\n` +
        "#{show block:html.frame;[\n" +
        item +
        "\n]}"
      );
    }),
);

const prelude = await getStorageItem<string>(`spaces/${spaceId.value}/prelude/main.typ`);

const typstState = await useTypst();

const pixelPerPoint = ref(window.devicePixelRatio);

const theme = useMaterialTheme()!;
const palette = computed(() => theme.value.palette);

function parseColor(color: Rgba): Rgb {
  return new Rgb(color.r, color.g, color.b);
}

const path = `spaces/${spaceId.value}/export.typ`;
const fileId = typstState.createSourceId(path, spaceId.value);

watchImmediate([pixelPerPoint, palette], ([pixelPerPoint, palette]) => {
  typstState.setTheme(
    fileId,
    new ThemeColors(
      parseColor(palette.background),
      parseColor(palette.onBackground),

      parseColor(palette.outline),
      parseColor(palette.outlineVariant),

      parseColor(palette.primary),
      parseColor(palette.onPrimary),
      parseColor(palette.primaryContainer),
      parseColor(palette.onPrimaryContainer),

      parseColor(palette.secondary),
      parseColor(palette.onSecondary),
      parseColor(palette.secondaryContainer),
      parseColor(palette.onSecondaryContainer),

      parseColor(palette.tertiary),
      parseColor(palette.onTertiary),
      parseColor(palette.tertiaryContainer),
      parseColor(palette.onTertiaryContainer),

      parseColor(palette.error),
      parseColor(palette.onError),
      parseColor(palette.errorContainer),
      parseColor(palette.onErrorContainer),
    ),
  );
});

try {
  const packages = await useInstalledPackages(spaceId.value);
  await Promise.all(packages.value.map((pkg) => installTypstPackage(pkg, spaceId.value)));
} catch (err) {
  console.error("Error installing packages:", err);
}

// console.log(dailyNotes.join("\n"));

typstState.insertSource(fileId, dailyNotes.join("\n"));

const { document, diagnostics } = typstState.renderHtml(
  fileId,
  dailyNotes.join("\n"),
  prelude || "",
);

const errors = diagnostics.filter((diagnostic) => diagnostic.severity === "error");

for (const error of errors) console.error(error);

const stickyNotes = await useStorageItem<Record<string, StickyNote>>(
  () => `spaces/${spaceId.value}/sticky/notes.json`,
  {},
);
const activeStickyNotes = ref<StickyNote[]>([]);
</script>

<template>
  <div class="flex gap-6 p-3">
    <aside class="expanded:flex max-h-80vh sticky top-16 hidden size-full max-w-80 self-start p-3">
      <md-outlined-card class="size-full">
        <span class="label-large m-3">Sticky Notes</span>

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
      </md-outlined-card>
    </aside>

    <main class="typst-document m-4 w-full flex-1 space-y-3 overflow-hidden">
      <section class="display-small flex items-center justify-between gap-2">
        <h1>{{ space.name }}</h1>
        <md-icon v-if="space.icon">{{ space.icon }}</md-icon>
      </section>

      <md-divider />

      <article v-if="document" v-html="document" />
    </main>
  </div>
</template>
