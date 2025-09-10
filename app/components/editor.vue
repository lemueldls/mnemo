<script setup lang="ts">
import { autocompletion, closeBrackets } from "@codemirror/autocomplete";

import {
  bracketMatching,
  defaultHighlightStyle,
  // foldGutter,
  indentOnInput,
  syntaxHighlighting,
} from "@codemirror/language";

import { highlightSelectionMatches } from "@codemirror/search";
import { EditorState } from "@codemirror/state";

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
import { LoroExtensions } from "loro-codemirror";
import { EphemeralStore } from "loro-crdt";
import { Rgb } from "mnemo-wasm";
import { ThemeColors, type FileId } from "mnemo-wasm";
import { match } from "ts-pattern";
import { normalizeKey } from "unstorage";

import type { NoteKind } from "~/composables/notes";
import type { Rgba } from "~~/modules/mx/types";

import { typstLanguage } from "~/lib/editor/language";
import { typstPlugin } from "~/lib/editor/widget";

const props = defineProps<{
  spaceId: string;
  kind: NoteKind;
  readonly?: boolean;
  locked?: boolean;
  faded?: boolean;
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

const preludeItem = await useStorageText(
  () => `spaces/${props.spaceId}/prelude/main.typ`,
);
const prelude = computed(() =>
  match(props.kind)
    .with("prelude", "task", () => "")
    .otherwise(() => preludeItem.value),
);

const { t } = useI18n();

const typstState = await useTypst();

const text = await useStorageText(fullPath);

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

  await watchImmediateAsync(fullPath, async (fullPath) => {
    const fileId = typstState.createFileId(fullPath);

    typstState.setPixelPerPt(fileId, window.devicePixelRatio);

    await watchImmediateAsync(palette, async (palette) => {
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

    watch(
      text,
      (text) => {
        typstState.insertFile(fileId, text);
        const state = createEditorState(fileId);
        view.setState(state);
      },
      { once: true, immediate: !ready },
    );
  });

  ready = true;
  queueMicrotask(() => emit("ready"));
});

const addSpaceBeforeClosingBracket = EditorView.inputHandler.of(
  (view, from, to, text) => {
    if (text === " ") {
      const state = view.state;
      const pos = from;
      const bracketPairs = { "(": ")", "[": "]", "{": "}", $: "$" };
      const before = state.doc.sliceString(
        pos - 1,
        pos,
      ) as keyof typeof bracketPairs;
      const after = state.doc.sliceString(
        pos,
        pos + 1,
      ) as keyof typeof bracketPairs;

      if (bracketPairs[before] && after === bracketPairs[before]) {
        // Insert a space before the closing bracket
        view.dispatch({
          changes: { from: pos, to: pos, insert: " " },
          selection: { anchor: pos + 1 },
        });
      }
    }

    return false;
  },
);

const doc = await useCrdt();
const undoManager = await useCrdtUndoManager();

function createEditorState(fileId: FileId): EditorState {
  const path = normalizeKey(fullPath.value);

  return EditorState.create({
    extensions: [
      typstPlugin(typstState, path, fileId, prelude, props.locked),
      typstLanguage(typstState),

      EditorView.lineWrapping,
      EditorView.editable.of(!props.readonly),
      EditorState.readOnly.of(props.readonly),

      placeholder("write."),
      highlightSpecialChars(),
      // foldGutter(),
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
      addSpaceBeforeClosingBracket,
      keymap.of(vscodeKeymap),

      LoroExtensions(
        doc,
        {
          ephemeral: new EphemeralStore(),
          user: { name: "You", colorClassName: "you" },
        },
        undoManager,
        (doc) => doc.getText(path),
      ),
    ],
  });
}

function reloadEditorWidgets(view: EditorView) {
  const { doc } = view.state;

  if (doc.length) {
    const from = 0;
    const to = 1;

    view?.dispatch({
      changes: { from, to, insert: doc.sliceString(from, to) },
    });
  }
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
    <div
      ref="container"
      :class="['editor', { editor__locked: locked, editor__faded: faded }]"
    />
  </div>
</template>

<style lang="scss">
.editor {
  @apply size-full overflow-hidden;

  .cm-editor {
    @apply body-large h-full outline-none;
  }

  .cm-scroller {
    @apply overflow-x-hidden overflow-y-scroll;
  }

  &__locked .cm-scroller {
    @apply overflow-hidden!;
  }

  &__faded {
    mask-image: linear-gradient(to bottom, black 50%, transparent 100%);
  }

  .cm-line {
    @apply p-0 px-[1px] text-[16px];

    font-family: var(--font-mono);
  }

  .cm-content {
    @apply caret-primary p-0;

    font-family: var(--font-mono);
  }

  .cm-selectionBackground,
  .cm-content ::selection {
    @apply text-tertiary;

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
    @apply bg-surface-container-lowest rounded-bl-0 max-w-1/3 m-0 rounded-lg border-none p-0 shadow;

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
}
</style>
