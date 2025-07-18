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
import { Rgb } from "mnemo-wasm";
import { ThemeColors, type FileId, type TypstState } from "mnemo-wasm";

import type { EditorStateConfig } from "@codemirror/state";
import type { NoteKind } from "~/composables/notes";
import type { Rgba } from "~~/modules/mx/types";

import { typstLanguage } from "~/lib/editor/language";
import { typst } from "~/lib/editor/widget";

import { EphemeralStore } from "loro-crdt";
import { LoroExtensions } from "loro-codemirror";
import { normalizeKey } from "unstorage";

const props = defineProps<{
  spaceId: string;
  kind: NoteKind;
  readonly?: boolean;
}>();

const pathId = defineModel<string>({ required: true });
const fullPath = computed(
  () => `spaces/${props.spaceId}/${props.kind}/${pathId.value}.typ`,
);

const pixelPerPoint = ref(window.devicePixelRatio);
// const pxToPt = (px: number) => px * window.devicePixelRatio * (72 / 96);

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

const containerRef = useTemplateRef("container");

const stateCache: { [key: string]: unknown } = {};

const preludeItem = await useStorageItem(
  () => `spaces/${props.spaceId}/prelude/main.typ`,
  "",
);
const prelude = computed(() =>
  props.kind === "prelude" ? "" : preludeItem.value,
);

const packages = await useInstalledPackages(() => props.spaceId);
watchImmediate(packages, async (packages) => {
  await Promise.all(packages.map((pkg) => installTypstPackage(pkg)));
});

const { t } = useI18n();

const typstState = await useTypst();

const text = await useStorageItem(fullPath, "");

onMounted(() => {
  const container = containerRef.value!;
  const view = new EditorView({
    parent: container,
    root: document,
    state: EditorState.create({
      extensions: [
        EditorState.readOnly.of(true),
        placeholder(t("components.editor.loading")),
      ],
    }),
  });

  let ready = false;

  watchImmediate(fullPath, (fullPath, oldFullPath) =>
    watch(
      text,
      (text) => {
        const fileId = typstState.insertFile(fullPath, text);

        if (oldFullPath)
          stateCache[oldFullPath] = view.state.toJSON({
            history: historyField,
          });

        const cache = stateCache[fullPath];
        const stateConfig = createStateConfig(typstState, fileId);

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
    ),
  );

  ready = true;
});

const addSpaceBeforeClosingBracket = EditorView.inputHandler.of(
  (view, from, to, text) => {
    if (text === " ") {
      const state = view.state;
      const pos = from;
      const before = state.doc.sliceString(
        pos - 1,
        pos,
      ) as keyof typeof bracketPairs;
      const after = state.doc.sliceString(
        pos,
        pos + 1,
      ) as keyof typeof bracketPairs;
      const bracketPairs = { "(": ")", "[": "]", "{": "}", $: "$" };

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

// const doc = await useCrdt();
// const undoManager = await useCrdtUndoManager();

// const name = await useStorageText("name", "");

function createStateConfig(
  typstState: TypstState,
  fileId: FileId,
): EditorStateConfig {
  return {
    extensions: [
      typst(typstState, fullPath, fileId, text, prelude),
      typstLanguage(typstState),

      EditorView.lineWrapping,
      // updateListenerExtension,

      EditorState.readOnly.of(props.readonly),

      placeholder("write."),
      highlightSpecialChars(),
      history(),
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

      // LoroExtensions(
      //   doc,
      //   {
      //     ephemeral: new EphemeralStore(),
      //     user: { name: name.value, colorClassName: "user1" },
      //   },
      //   undoManager,
      //   (doc) => {
      //     const item = doc.getText(normalizeKey(fullPath.value));
      //     if (!item.length) item.update(text.value);

      //     return item;
      //   },
      // ),
    ],
  };
}

const selectionBackground = computed(() => {
  const { r, g, b } = palette.value.onTertiary;

  return `rgba(${r},${g},${b},0.5)`;
});
const selectionMatch = computed(() => {
  const { r, g, b } = palette.value.tertiaryContainer;

  return `rgba(${r},${g},${b},0.5)`;
});
const activeLineBackground = computed(() => {
  const { primaryContainer } = palette.value;

  return `rgba(${primaryContainer.r},${primaryContainer.g},${primaryContainer.b},0.25)`;
});
const renderHoverBackground = computed(() => {
  const { secondaryContainer } = palette.value;

  return `rgba(${secondaryContainer.r},${secondaryContainer.g},${secondaryContainer.b},0.25)`;
});
</script>

<template>
  <div class="size-full overflow-hidden">
    <div ref="container" class="editor" />
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

  .cm-selectionMatch {
    @apply bg-on-tertiary-container;

    color: v-bind(selectionMatch) !important;
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
    @apply bg-surface-container-lowest rounded-bl-0 m-0 max-w-xl rounded-lg border-none p-0 shadow;

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
    @apply flex flex-col gap-1 p-1 text-sm;

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
    @apply inline-block cursor-text align-top transition-colors hover:rounded;

    -webkit-user-drag: none;
    -moz-user-drag: none;

    &:hover {
      background-color: v-bind(renderHoverBackground);
    }
  }
}
</style>
