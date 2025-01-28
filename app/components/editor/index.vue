<script setup lang="ts">
import {
  highlightSpecialChars,
  drawSelection,
  dropCursor,
  rectangularSelection,
  crosshairCursor,
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

// import { redFromArgb } from "@material/material-color-utilities";

import { Rgb } from "mnemo-wasm";

import { typst } from "./widget";
import { underlineKeymap } from "./underline";

import { typstLanguage } from "./languague";

import type { Rgba } from "@material/material-color-utilities";

import type { EditorStateConfig } from "@codemirror/state";
import { ThemeColors, type TypstState, type FileId } from "mnemo-wasm";
import type { Package } from "~~/server/api/list-packages";

const props = defineProps<{
  kind: NoteKind;
  spaceId: string;
  readonly?: boolean;
}>();
// const path = useVModel(props, "modelValue", emit);
const path = defineModel<string>({ required: true });

const pixelPerPoint = ref(window.devicePixelRatio);
const pxToPt = (px: number) => px * window.devicePixelRatio * (72 / 96);

const { palette } = useMaterialTheme()!;

function parseColor(color: Rgba): Rgb {
  return new Rgb(color.r, color.g, color.b);
}

watchEffect(async () => {
  const typstState = await useTypst();

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
});

const containerRef = useTemplateRef("container");

// const updateListenerExtension = EditorView.updateListener.of(async (update) => {
//   // if (update.docChanged) await sync();
// });

const stateCache: { [key: string]: unknown } = {};

const storageItem = await useRefStorageItem(
  computed(() => `spaces/${props.spaceId}/${props.kind}/${path.value}.typ`),
  "",
);

// const storageItem = ref("");

onMounted(() => {
  const container = containerRef.value;
  const view = new EditorView({
    parent: container,
    state: EditorState.create({
      extensions: [EditorState.readOnly.of(true), placeholder("Loading...")],
    }),
  });

  watchImmediate(
    () => path.value,
    async (path, oldPath) => {
      if (oldPath) await until(storageItem).changed();

      // const note = await useStorageItem(
      //   `spaces/${props.spaceId}/${props.kind}/${path}.typ`,
      //   "",
      // );

      // watchOnce(note, (note) => {
      //   storageItem.value = note;
      // });

      // watchEffect(() => {
      //   console.log({ path, note: note.value });
      // });

      // watchImmediate(note, (note) => {
      //   storageItem.value = note;
      // });

      const typstState = await useTypst();

      const packages = await useStorageItem<Package[]>(
        `spaces/${props.spaceId}/packages.json`,
        [],
      );
      watchImmediate(packages, async (packages) => {
        await Promise.all(packages.map((pkg) => installTypstPackage(pkg)));
      });

      const text = storageItem.value;
      console.log({ text });
      const fileId = typstState.insertFile(path, text);

      if (oldPath) stateCache[oldPath] = view.state.toJSON();

      const cache = stateCache[path];
      const stateConfig = createStateConfig(typstState, path, fileId);

      if (cache) view.setState(EditorState.fromJSON(cache, stateConfig));
      else {
        stateConfig.doc = text;
        view.setState(EditorState.create(stateConfig));
      }
    },
  );
});

function createStateConfig(
  typstState: TypstState,
  path: string,
  fileId: FileId,
): EditorStateConfig {
  return {
    extensions: [
      typst(typstState, storageItem, fileId),
      typstLanguage(),
      underlineKeymap,

      EditorView.lineWrapping,
      // updateListenerExtension,

      EditorState.readOnly.of(props.readonly),

      placeholder("Go on."),
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
      EditorView.contentAttributes.of({ spellcheck: "true" }),
      autocompletion({ activateOnTyping: true }),
      rectangularSelection(),
      crosshairCursor(),
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

const { secondaryContainer } = palette;

const activeLineBackground = `rgba(${secondaryContainer.r},${secondaryContainer.g},${secondaryContainer.b},0.25)`;
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
    /* font-family: "Iosevka Quasi Custom"; */
    font-family: "Iosevka Book Web";
    /* font-style: normal;
    font-display: swap;
    font-stretch: normal;
    font-variant-ligatures: none;
    font-kerning: none; */
    font-kerning: normal !important;
    // letter-spacing: 0;
    // word-spacing: 4px;
    /* font-feature-settings: "liga" 0; */
    /* line-height: 1.5; */
  }

  .cm-content {
    @apply caret-m3-primary p-0;

    /* font-family: "Iosevka Quasi Custom"; */
    font-family: "Iosevka Book Web";
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

  .cm-activeLine {
    // @apply px-1;

    background-color: v-bind(activeLineBackground);
  }

  .typst-render {
    display: inline;
    // vertical-align: bottom;
    cursor: text;
    user-drag: none;
    user-select: none;
    // pointer-events: none;
    /* overflow: hidden; */
    /* display: flex; */

    /* &:hover {
      background-color: v-bind(activeLineBackground);
    } */
  }

  code {
    font-family: "Iosevka Custom", monospace;
  }
}
</style>
