<script setup lang="ts">
import { autocompletion, closeBrackets } from "@codemirror/autocomplete";

import {
  bracketMatching,
  defaultHighlightStyle,
  // foldGutter,
  indentOnInput,
  syntaxHighlighting,
} from "@codemirror/language";

import { lintGutter } from "@codemirror/lint";
import { highlightSelectionMatches } from "@codemirror/search";
import { EditorState, type EditorStateConfig } from "@codemirror/state";
import { history, historyField } from "@codemirror/commands";

import {
  crosshairCursor,
  drawSelection,
  dropCursor,
  EditorView,
  highlightSpecialChars,
  keymap,
  placeholder,
  rectangularSelection,
} from "@codemirror/view";

import { vscodeKeymap } from "@replit/codemirror-vscode-keymap";
// import { LoroExtensions } from "loro-codemirror";
// import { EphemeralStore } from "loro-crdt";
import { Rgb } from "mnemo-wasm";
import { ThemeColors, type FileId } from "mnemo-wasm";
import { match } from "ts-pattern";
import { normalizeKey } from "unstorage";

import type { NoteKind } from "~/composables/notes";
import type { Rgba } from "~~/modules/mx/types";

import { typstPlugin } from "~/lib/editor/plugin";
import { LRUCache } from "lru-cache";

const props = defineProps<{
  spaceId: string;
  kind: NoteKind;
  readonly?: boolean;
  locked?: boolean;
}>();

const emit = defineEmits<{ (e: "ready"): void }>();

const pathId = defineModel<string>({ required: true });
const fullPath = computed(
  () => `spaces/${props.spaceId}/${props.kind}/${pathId.value}.typ`,
);

const theme = useMaterialTheme()!;
const palette = computed(() => theme.value.palette);

function parseColor(color: Rgba): Rgb {
  return new Rgb(color.r, color.g, color.b);
}

const containerRef = useTemplateRef("container");

const topFade = ref(0);
const bottomFade = ref(0);
const maxFade = 32;

const preludeItem = await useStorageText(
  () => `spaces/${props.spaceId}/prelude/main.typ`,
);
const prelude = computed(() =>
  match(props.kind)
    .with("prelude", "task", () => "")
    .otherwise(() => preludeItem.value),
);

const { t, locale } = useI18n();

const typstState = await useTypst();

const text = await useStorageText(fullPath);

const stateCache = new LRUCache<string, EditorState>({ max: 3 });

onMounted(async () => {
  const container = containerRef.value!;
  const view = new EditorView({
    parent: container,
    root: document,
    state: EditorState.create({
      extensions: [
        EditorView.editable.of(false),
        EditorState.readOnly.of(true),
        placeholder(t("components.editor.loading")),
        lintGutter(),
      ],
    }),
  });

  let ready = false;

  try {
    const packages = await useInstalledPackages(() => props.spaceId);
    await watchImmediateAsync(packages, async (packages) => {
      await Promise.all(packages.map((pkg) => installTypstPackage(pkg)));

      if (ready) reloadEditorWidgets(view);
    });
  } catch (err) {
    console.error("Error installing packages:", err);
  }

  watchImmediate(fullPath, (fullPath, oldFullPath) => {
    const fileId = typstState.createFileId(fullPath);

    typstState.setPixelPerPt(fileId, window.devicePixelRatio);

    watchImmediate(palette, (palette) => {
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

      if (ready) reloadEditorWidgets(view);
    });

    watchImmediate(locale, (locale) => {
      typstState.setLocale(fileId, locale);

      if (ready) reloadEditorWidgets(view);
    });

    watch(
      text,
      (text) => {
        typstState.insertFile(fileId, text);

        if (oldFullPath)
          stateCache.set(
            oldFullPath,
            view.state.toJSON({
              history: historyField,
            }),
          );

        const cache = stateCache.get(fullPath);
        const stateConfig = createStateConfig(fileId, view);

        if (cache)
          view.setState(
            EditorState.fromJSON(cache, stateConfig, {
              history: historyField,
            }),
          );
        else {
          stateConfig.doc = text;
          view.setState(EditorState.create(stateConfig));
        }
      },
      { once: true, immediate: !ready },
    );
  });

  ready = true;
  queueMicrotask(() => emit("ready"));

  const { scrollDOM } = view;
  const scrollHeight = useScrollHeight(scrollDOM);
  const { y: scrollY } = useScroll(scrollDOM);
  const { height } = useElementSize(scrollDOM);

  watchImmediate(
    [scrollHeight, scrollY, height],
    ([scrollHeight, scrollY, height]) => {
      if (!scrollDOM) return;

      topFade.value = Math.min(scrollY, maxFade);
      bottomFade.value = Math.min(scrollHeight - scrollY - height, maxFade);
    },
  );
});

// const doc = await useCrdt();
// const undoManager = await useCrdtUndoManager();

function createStateConfig(
  fileId: FileId,
  view: EditorView,
): EditorStateConfig {
  const path = normalizeKey(fullPath.value);

  return {
    extensions: [
      EditorView.exceptionSink.of((error) => {
        console.error(error);

        queueMicrotask(() => {
          const stateConfig = createStateConfig(fileId, view);
          stateConfig.doc = text.value;
          view.setState(EditorState.create(stateConfig));
        });
      }),

      typstPlugin(fileId, path, text, prelude, props.locked, typstState),

      EditorView.lineWrapping,
      EditorView.editable.of(!props.readonly),
      EditorState.readOnly.of(props.readonly),

      placeholder("write."),
      highlightSpecialChars(),
      // foldGutter(),
      lintGutter(),
      history(),
      drawSelection(),
      dropCursor(),
      EditorState.allowMultipleSelections.of(true),
      indentOnInput(),
      syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
      bracketMatching(),
      closeBrackets(),
      autocompletion(),
      rectangularSelection(),
      crosshairCursor(),
      highlightSelectionMatches(),

      keymap.of(vscodeKeymap),
    ],
  };
}

function reloadEditorWidgets(view: EditorView) {
  view.dispatch({ changes: [{ from: 0, insert: " " }] });
  view.dispatch({ changes: [{ from: 0, to: 1 }] });
}

const selectionBackground = computed(() => {
  const { r, g, b } = palette.value.tertiaryContainer;

  return `rgba(${r},${g},${b},0.25)`;
});
const selectionMatchBackground = computed(() => {
  const { r, g, b } = palette.value.onTertiary;

  return `rgba(${r},${g},${b},0.5)`;
});
const activeLineBackground = computed(() => {
  const { secondaryContainer } = palette.value;

  return `rgba(${secondaryContainer.r},${secondaryContainer.g},${secondaryContainer.b},0.75)`;
});
const renderHoverBackground = computed(() => {
  const { secondaryContainer } = palette.value;

  return `rgba(${secondaryContainer.r},${secondaryContainer.g},${secondaryContainer.b},0.25)`;
});
</script>

<template>
  <div class="size-full overflow-hidden">
    <div ref="container" :class="['editor', { editor__locked: locked }]" />
  </div>
</template>

<style lang="scss">
.editor {
  @apply size-full overflow-hidden pb-2 pr-2;

  .cm-editor {
    @apply body-large h-full outline-none;
  }

  .cm-scroller {
    @apply overflow-x-hidden overflow-y-scroll;
  }

  &__locked .cm-scroller {
    @apply overflow-hidden!;
  }

  .cm-scroller {
    mask-image: linear-gradient(
      to bottom,
      transparent 0%,
      rgba(0, 0, 0, 0.25) calc(v-bind(topFade) / 2 * 1px),
      black calc(v-bind(topFade) * 1px),
      black calc(100% - v-bind(bottomFade) * 1px),
      rgba(0, 0, 0, 0.25) calc(100% - v-bind(bottomFade) / 2 * 1px),
      transparent 100%
    );
  }

  .cm-line {
    @apply p-0 px-[1px] text-[16px];

    font-family: var(--font-mono);
  }

  .cm-content {
    @apply caret-primary p-0;

    font-family: var(--font-mono);
  }

  // .cm-selectionBackground,
  // .cm-content ::selection {
  //   @apply text-tertiary;

  //   background-color: v-bind(selectionBackground) !important;
  // }

  .cm-panels {
    @apply bg-surface-variant text-on-surface-variant border-outline-variant;
  }

  .cm-textfield {
    @apply text-on-surface-variant border-outline rounded-lg bg-transparent;
  }

  button,
  .cm-button {
    @apply bg-surface-container-high hover:bg-surface-container-highest border-outline text-on-surface rounded-lg bg-none;
  }

  label {
    @apply inline-flex items-center;
  }

  input[type="checkbox"] {
    @apply bg-primary! bg-primary! border-outline text-on-surface rounded-lg bg-none;
  }

  button[name="close"] {
    @apply size-5;
  }

  .cm-selectionMatch {
    @apply text-tertiary;

    background-color: v-bind(selectionMatchBackground) !important;
  }

  .cm-cursor {
    @apply border-primary;
  }

  .cm-activeLine {
    background-color: v-bind(activeLineBackground);
  }

  .cm-lintPoint-error::after {
    @apply border-b-error;
  }

  .cm-lintRange {
    @apply decoration-underline bg-none decoration-wavy;
  }

  .cm-lintRange-error {
    @apply decoration-error;
  }

  /* .cm-lintRange-hint {
    @apply decoration-error;
  } */

  .cm-tooltip {
    @apply bg-surface-container-lowest max-w-1/3 m-0 rounded-lg border-none p-0 shadow;

    font-family: var(--font-mono);

    [aria-selected="true"] {
      @apply bg-secondary-container! text-on-secondary-container!;
    }

    li {
      @apply p-1!;

      font-family: var(--font-mono);
    }
  }

  .cm-tooltip-lint {
    @apply -translate-x-1;

    li {
      @apply first:b-t-2 p-1! first:rounded-t last:rounded-b;
    }
  }

  .cm-tooltip-autocomplete {
    @apply z-10 flex flex-col gap-1 p-1 text-sm;

    li {
      @apply rounded;
    }
  }

  .cm-tooltip-hover {
    @apply p-1;
  }

  .cm-completionIcon {
    @apply translate-y-0.5;
  }

  .cm-completionIcon-syntax::after {
    content: "code";
    font-family: var(--md-icon-font);
  }

  .cm-completionIcon-function::after {
    content: "function";
    font-family: var(--md-icon-font);
  }

  .cm-completionIcon-type::after {
    content: "category";
    font-family: var(--md-icon-font);
  }

  .cm-completionIcon-param::after {
    content: "settings";
    font-family: var(--md-icon-font);
  }

  .cm-completionIcon-constant::after {
    content: "special_character";
    font-family: var(--md-icon-font);
  }

  .cm-completionIcon-path::after {
    content: "folder";
    font-family: var(--md-icon-font);
  }

  .cm-completionIcon-package::after {
    content: "package";
    font-family: var(--md-icon-font);
  }

  .cm-completionIcon-label::after {
    content: "label";
    font-family: var(--md-icon-font);
  }

  .cm-completionIcon-font::after {
    content: "font_download";
    font-family: var(--md-icon-font);
  }

  .cm-completionIcon-symbol::after {
    content: "tag";
    font-family: var(--md-icon-font);
  }

  /* .cm-tooltip-above {
    @apply rounded-bl-0;
  }

  .cm-tooltip-below {
    @apply rounded-tl-0;
  } */

  .cm-diagnostic {
    @apply b-l-6;
  }

  .cm-diagnostic-error {
    @apply text-error border-error;
  }

  .cm-diagnostic-hint {
    @apply text-outline border-outline;
  }

  .cm-gutters {
    @apply border-none bg-transparent;
  }

  .cm-gutter {
    @apply w-2;
  }

  .cm-gutter-lint .cm-gutterElement {
    @apply p-x-0.75 p-0;
  }

  .cm-lint-marker {
    @apply size-full content-none;
  }

  .cm-lint-marker-error {
    @apply bg-error;
  }

  .cm-lint-marker-warning {
    @apply bg-secondary;
  }

  .typst-render {
    @apply inline-block w-full align-top;

    -webkit-user-drag: none;
    -moz-user-drag: none;
  }

  .cm-content[contenteditable="true"] {
    .typst-render {
      @apply cursor-text transition-colors hover:rounded;

      &:hover {
        background-color: v-bind(renderHoverBackground);
      }
    }
  }

  .cm-highlight-comment {
    @apply text-outline;
  }

  .cm-highlight-punctuation {
    @apply text-on-surface-variant;
  }

  .cm-highlight-escape {
    @apply text-outline;
  }

  .cm-highlight-strong {
    @apply font-bold;
  }

  .cm-highlight-emph {
    @apply italic;
  }

  .cm-highlight-link {
    @apply text-primary underline;
  }

  .cm-highlight-raw {
    @apply text-secondary;
  }

  .cm-highlight-label {
    @apply text-on-primary-container;
  }

  .cm-highlight-ref {
    @apply text-on-primary-container;
  }

  .cm-highlight-heading {
    @apply text-on-secondary-container;
  }

  .cm-highlight-list-marker {
    @apply font-bold;
  }

  .cm-highlight-list-term {
    @apply font-bold;
  }

  .cm-highlight-math-delimiter {
    @apply text-outline;
  }

  .cm-highlight-math-operator {
    @apply text-on-error-container;
  }

  .cm-highlight-keyword {
    @apply text-primary;
  }

  .cm-highlight-operator {
    @apply text-on-error-container;
  }

  .cm-highlight-number {
    @apply text-tertiary;
  }

  .cm-highlight-string {
    @apply text-tertiary;
  }

  .cm-highlight-function {
    @apply text-on-primary-container;
  }

  .cm-highlight-interpolated {
    @apply text-on-secondary-container;
  }

  .cm-highlight-error {
    @apply text-error;
  }
}
</style>
