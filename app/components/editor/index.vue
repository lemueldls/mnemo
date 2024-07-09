<script setup lang="ts">
import {
  // lineNumbers,
  // highlightActiveLineGutter,
  highlightSpecialChars,
  drawSelection,
  dropCursor,
  rectangularSelection,
  crosshairCursor,
  // highlightActiveLine,
  keymap,
  placeholder,
  Decoration,
  EditorView,
  ViewPlugin,
  WidgetType,
} from "@codemirror/view";

import { EditorState } from "@codemirror/state";
import {
  // foldGutter,
  indentOnInput,
  syntaxTree,
  syntaxHighlighting,
  defaultHighlightStyle,
  bracketMatching,
  foldKeymap,
  LRLanguage,
} from "@codemirror/language";
import {
  history,
  defaultKeymap,
  historyKeymap,
  indentWithTab,
} from "@codemirror/commands";
import { highlightSelectionMatches, searchKeymap } from "@codemirror/search";
import {
  closeBrackets,
  autocompletion,
  closeBracketsKeymap,
  completionKeymap,
  type Completion,
  type CompletionContext,
  type CompletionResult,
} from "@codemirror/autocomplete";
import { lintKeymap } from "@codemirror/lint";

import { redFromArgb } from "@material/material-color-utilities";

import { Rgb } from "mnemo-wasm";

import { useTypst } from "./state";

import { typst } from "./widget";
import { underlineKeymap } from "./underline";

import { typstLanguage } from "./languague";

import type { TypstState } from "mnemo-wasm";

import type { Rgba } from "@material/material-color-utilities";

import type { EditorStateConfig } from "@codemirror/state";

const emit = defineEmits<{
  (event: "update:modelValue", value: string): void;
}>();
const properties = defineProps<{ space: string; modelValue: string }>();
const path = useVModel(properties, "modelValue", emit);

const pixelPerPoint = ref(1);

const pxToPt = (px: number) => px * window.devicePixelRatio * (72 / 96);

const { palette } = useMaterialTheme()!;

function parseColor (color: Rgba): Rgb {
  return new Rgb(color.r, color.g, color.b);
}

watchEffect(async () => {
  const typstState = await useTypst();

  typstState.color = parseColor(palette.onBackground);
  typstState.stroke = parseColor(palette.outline);
  typstState.pt = pixelPerPoint.value;
  typstState.size = 16 / pixelPerPoint.value;
  typstState.h1 = parseColor(palette.onPrimaryContainer);
  typstState.h2 = parseColor(palette.onSecondaryContainer);
  typstState.h3 = parseColor(palette.onTertiaryContainer);
  typstState.h4 = parseColor(palette.primary);
  typstState.h5 = parseColor(palette.secondary);
  typstState.h6 = parseColor(palette.tertiary);
});

const container = shallowRef<HTMLDivElement>();

// const updateListenerExtension = EditorView.updateListener.of(async (update) => {
//   // if (update.docChanged) await sync();
// });

const stateCache: { [key: string]: unknown } = {};

onMounted(() => {
  const parent = container.value;
  const view = new EditorView({
    parent,
    state: EditorState.create({
      extensions: [EditorState.readOnly.of(true), placeholder("Loading...")],
    }),
  });

  watchImmediate(path, async (path, oldPath) => {
    const typstState = await useTypst();

    const text = await readSpaceFile(properties.space, path);
    typstState.setMain(path, text);

    if (oldPath) stateCache[oldPath] = view.state.toJSON();

    const cache = stateCache[path];
    const stateConfig = createStateConfig(typstState, path);

    if (cache) view.setState(EditorState.fromJSON(cache, stateConfig));
    else {
      stateConfig.doc = text;
      view.setState(EditorState.create(stateConfig));
    }
  });
});

function createStateConfig (
  typstState: TypstState,
  path: string,
): EditorStateConfig {
  return {
    extensions: [
      typst(typstState, properties.space, path),
      typstLanguage(),
      underlineKeymap,

      EditorView.lineWrapping,
      // updateListenerExtension,

      placeholder("Go on."),
      // lineNumbers(),
      // highlightActiveLineGutter(),
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
      autocompletion({ activateOnTyping: false }),
      rectangularSelection(),
      crosshairCursor(),
      // highlightActiveLine(),
      highlightSelectionMatches(),
      keymap.of([
        indentWithTab,
        ...closeBracketsKeymap,
        ...defaultKeymap,
        ...searchKeymap,
        ...historyKeymap,
        ...foldKeymap,
        ...completionKeymap,
        ...lintKeymap,
      ]),
    ],
  };
}
</script>

<template>
  <div ref="container" class="editor" />
</template>

<style lang="scss">
.editor {
  @apply overflow-auto;

  // &,
  // * {
  //   font-kerning: none !important;
  //   font-variant-ligatures: none !important;
  // }

  .cm-editor {
    @apply outline-none m3-body-large h-full;
  }

  .cm-line {
    // @apply p-0 pl-1px text-[16px] tracking-0 word-spacing-[4px] [font-kerning:none];
    @apply p-0 pl-1px text-[16px];
    font-family: "Iosevka Quasi Custom";
    font-style: normal;
    font-display: swap;
    font-stretch: normal;
    font-variant-ligatures: none;
    font-kerning: none;
    letter-spacing: 0;
    // word-spacing: 4px;
    font-feature-settings: "liga" 0;
    line-height: 1.5;
  }

  .cm-content {
    @apply caret-m3-primary p-0;

    font-family: "Iosevka Quasi Custom";
  }

  .cm-selectionBackground {
    @apply text-m3-on-surface-variant;

    background: var(--md-sys-color-surface-container-highest) !important;
  }

  .cm-selectionMatch {
    @apply bg-m3-tertiary-container text-m3-on-tertiary-container;
  }

  .cm-cursor {
    @apply border-m3-primary;
  }

  code {
    font-family: "Iosevka Custom", monospace;
  }
}
</style>
