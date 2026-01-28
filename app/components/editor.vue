<script setup lang="ts">
import { autocompletion, closeBrackets } from "@codemirror/autocomplete";
import { history, historyField } from "@codemirror/commands";

import {
  bracketMatching,
  defaultHighlightStyle,
  // foldGutter,
  indentOnInput,
  syntaxHighlighting,
} from "@codemirror/language";

import { lintGutter } from "@codemirror/lint";
import { highlightSelectionMatches } from "@codemirror/search";
import { Compartment, EditorState, type EditorStateConfig } from "@codemirror/state";

import {
  EditorView,
  crosshairCursor,
  drawSelection,
  dropCursor,
  highlightSpecialChars,
  placeholder,
  rectangularSelection,
} from "@codemirror/view";

// import { LoroExtensions } from "loro-codemirror";
// import { EphemeralStore } from "loro-crdt";
import { type FileId, Rgb, ThemeColors } from "mnemo-wasm";
import { LRUCache } from "lru-cache";
import type { NoteKind } from "~/composables/notes";
import type { Rgba } from "~~/modules/mx/types";
import { match } from "ts-pattern";
import { normalizeKey } from "unstorage";

import { typstPlugin } from "~/lib/editor/plugin";

const props = defineProps<{
  spaceId: string;
  kind: NoteKind;
  readonly?: boolean;
  locked?: boolean;
}>();

const emit = defineEmits<{ (e: "ready"): void }>();

const pathId = defineModel<string>({ required: true });
const fullPath = computed(() => `spaces/${props.spaceId}/${props.kind}/${pathId.value}.typ`);

const theme = useMaterialTheme()!;
const palette = computed(() => theme.value.palette);

function parseColor(color: Rgba): Rgb {
  return new Rgb(color.r, color.g, color.b);
}

const containerRef = useTemplateRef("container");

const topFade = ref(0);
const bottomFade = ref(0);
const maxFade = 32;

const preludeItem = await useStorageText(() => `spaces/${props.spaceId}/prelude/main.typ`);
const prelude = computed(() =>
  match(props.kind)
    .with("prelude", "task", () => "")
    .otherwise(() => preludeItem.value),
);

const { t, locale } = useSharedI18n();

const typstState = await useTypst();
// const idsToCleanup = new Set<FileId>();

const text = await useStorageText(fullPath);

const stateCache = new LRUCache<string, EditorState>({ max: 3 });

const placeholderCompartment = new Compartment();

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

  watchImmediate(
    () => t("components.editor.placeholder"),
    (content) => {
      view.dispatch({
        effects: placeholderCompartment.reconfigure(placeholder(content)),
      });
    },
  );

  watchImmediate(fullPath, (fullPath, oldFullPath) => {
    const fileId = typstState.createSourceId(fullPath, props.spaceId);
    // idsToCleanup.add(fileId);

    // typstState.setPixelPerPt(fileId, window.devicePixelRatio);
    // useEventListener(window, "resize", () => {
    //   typstState.setPixelPerPt(fileId, window.devicePixelRatio);
    // });

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
        typstState.insertSource(fileId, text);

        if (oldFullPath) {
          stateCache.set(
            oldFullPath,
            view.state.toJSON({
              history: historyField,
            }),
          );
        }

        const cache = stateCache.get(fullPath);
        const stateConfig = createStateConfig(fileId, view);

        if (cache) {
          view.setState(
            EditorState.fromJSON(cache, stateConfig, {
              history: historyField,
            }),
          );
        } else {
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

  watchImmediate([scrollHeight, scrollY, height], ([scrollHeight, scrollY, height]) => {
    if (!scrollDOM) return;

    topFade.value = Math.min(scrollY, maxFade);
    bottomFade.value = Math.min(scrollHeight - scrollY - height, maxFade);
  });
});

// onUnmounted(() => {
//   console.log({ idsToCleanup });
//   for (const id of idsToCleanup) {
//     typstState.removeFile(id);
//   }
// });

// const doc = await useCrdt();
// const undoManager = await useCrdtUndoManager();

function createStateConfig(fileId: FileId, view: EditorView): EditorStateConfig {
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

      typstPlugin(fileId, props.spaceId, path, text, prelude, props.locked, typstState),

      EditorView.lineWrapping,
      EditorView.editable.of(!props.readonly),
      EditorState.readOnly.of(props.readonly),

      placeholderCompartment.of(placeholder(t("components.editor.placeholder"))),

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
    ],
  };
}

function reloadEditorWidgets(view: EditorView) {
  view.dispatch({ changes: [{ from: 0, insert: "\n" }] });
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
    <div ref="container" :class="['editor typst-document', { editor__locked: locked }]" />
  </div>
</template>

<style lang="scss">
.editor {
  @apply size-full overflow-hidden pr-2 pb-2;

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

    container-name: content;
    container-type: inline-size;
  }

  .cm-placeholder {
    @apply text-outline;
  }

  .cm-selectionBackground {
    // @apply text-tertiary;

    background-color: v-bind(selectionBackground) !important;
  }

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
    // @apply text-tertiary;

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
    @apply bg-surface-container-lowest m-0 max-h-1/3 max-w-1/3 overflow-auto rounded-lg border-none p-0 font-mono shadow;

    font-family: var(--font-mono), var(--font-math);

    li[aria-selected="true"] {
      @apply bg-secondary-container! text-on-secondary-container!;
    }

    li[role="option"] {
      @apply p-1! font-mono;
    }

    pre {
      @apply bg-surface-container rounded;
    }

    code {
      @apply bg-surface-container rounded px-1 font-mono text-wrap;
    }
  }

  .cm-tooltip-hover {
    @apply p-1;
  }

  // .cm-tooltip-below {
  //   @apply rounded-tl-0;
  // }

  // .cm-tooltip-above {
  //   @apply rounded-bl-0;
  // }

  .cm-tooltip-lint {
    .cm-diagnostic {
      @apply first:rounded-t last:rounded-b;
    }
  }

  .cm-tooltip-autocomplete {
    @apply z-10 flex flex-col gap-1 p-1 text-sm;

    li {
      @apply rounded;
    }
  }

  // .cm-tooltip-hover {
  //   @apply p-1;
  // }

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

  .cm-tooltip-section:not(:first-child) {
    @apply mt-1 rounded-tl border-none;
  }

  .cm-diagnostic {
    @apply b-l-6;
  }

  .cm-diagnostic-error {
    @apply text-error border-error;
  }

  .cm-diagnostic-warning {
    @apply text-secondary border-secondary;
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

  .typst-hints {
    @apply text-secondary body-small text-wrap;

    li {
      @apply text-wrap;
    }
  }

  .typst-render {
    @apply inline-block max-w-full align-top;

    -webkit-user-drag: none;
    -moz-user-drag: none;

    .typst-doc {
      @apply inline-block w-full;
    }
  }

  .cm-content[contenteditable="true"] {
    .typst-render {
      @apply rounded transition-colors;

      &:hover {
        background-color: v-bind(renderHoverBackground);
      }
    }

    .typst-frame {
      @apply cursor-text;
    }
  }

  .typ-comment {
    @apply text-outline;
  }

  .typ-punct {
    @apply text-on-surface-variant;
  }

  .typ-escape {
    @apply text-outline;
  }

  .typ-strong {
    @apply font-bold;
  }

  .typ-emph {
    @apply italic;
  }

  .typ-link {
    @apply text-primary underline;
  }

  .typ-raw {
    @apply text-tertiary;
  }

  .typ-label {
    @apply text-on-primary-container;
  }

  .typ-ref {
    @apply text-on-primary-container;
  }

  .typ-heading {
    @apply text-on-secondary-container;
  }

  .typ-heading-level-1 {
    @apply text-primary headline-large font-normal!;
  }

  .typ-heading-level-2 {
    @apply text-secondary headline-medium font-normal;
  }

  .typ-heading-level-3 {
    @apply text-tertiary headline-small font-normal;
  }

  .typ-heading-level-4 {
    @apply text-primary title-large font-normal;
  }

  .typ-heading-level-5 {
    @apply text-secondary title-medium font-semibold;
  }

  .typ-heading-level-6 {
    @apply text-tertiary title-small font-semibold;
  }

  .typ-marker {
    @apply text-on-surface-variant font-bold;
  }

  .typ-term {
    @apply font-bold;
  }

  .typ-math-delim {
    @apply text-on-surface-variant;
  }

  .typ-math-op {
    @apply text-on-error-container;
  }

  .typ-key {
    @apply text-primary;
  }

  .typ-op {
    @apply text-on-error-container;
  }

  .typ-num {
    @apply text-tertiary;
  }

  .typ-str {
    @apply text-tertiary;
  }

  .typ-func {
    @apply text-on-primary-container;
  }

  .typ-pol {
    @apply text-on-secondary-container;
  }

  .typ-error {
    @apply text-error;
  }
}
</style>
