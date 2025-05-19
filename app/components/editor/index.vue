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
  historyField,
} from "@codemirror/commands";
import { highlightSelectionMatches, searchKeymap } from "@codemirror/search";
import {
  closeBrackets,
  autocompletion,
  closeBracketsKeymap,
  completionKeymap,
} from "@codemirror/autocomplete";
import { lintKeymap } from "@codemirror/lint";

import { Rgb } from "mnemo-wasm";

import { typst } from "./widget";
import { underlineKeymap } from "./underline";

import { typstLanguage } from "./languague";

import type { Rgba } from "@material/material-color-utilities";

import type { EditorStateConfig } from "@codemirror/state";
import { ThemeColors, type TypstState, type FileId } from "mnemo-wasm";

const props = defineProps<{
  kind: NoteKind;
  spaceId: string;
  readonly?: boolean;
}>();

const path = defineModel<string>({ required: true });

const pixelPerPoint = ref(window.devicePixelRatio);
// const pxToPt = (px: number) => px * window.devicePixelRatio * (72 / 96);

const theme = useMaterialTheme();
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

const containerRef = useTemplateRef("container");

const stateCache: { [key: string]: unknown } = {};

const storageItem = ref() as Ref<Ref<string>>;
storageItem.value = ref("");

const preludeItem = await useRefStorageItem(
  computed(() => `spaces/${props.spaceId}/prelude/main.typ`),
  "",
);
const prelude = computed(() =>
  props.kind === "prelude" ? "" : preludeItem.value,
);

onMounted(() => {
  const container = containerRef.value;
  const view = new EditorView({
    parent: container,
    state: EditorState.create({
      extensions: [EditorState.readOnly.of(true), placeholder("Loading...")],
    }),
  });

  watchImmediate(
    [() => path.value, () => props.spaceId],
    async ([path, spaceId], [oldPath, oldSpace]) => {
      // console.log({ path, spaceId, oldPath, oldSpace });
      storageItem.value = await useStorageItem(
        `spaces/${oldSpace || spaceId}/${props.kind}/${path}.typ`,
        "",
      );

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

      const packages = await useInstalledPackages(spaceId);
      // check if spamming
      watchImmediate(packages, async (packages) => {
        await Promise.all(
          packages
            .filter((pkg) => pkg.name !== "suiji")
            .map((pkg) => installTypstPackage(pkg)),
        );
      });

      const text = storageItem.value.value;
      // if (!text) console.log("[DELETING]");
      // console.log({ path, text });
      const fileId = typstState.insertFile(path, text);

      if (oldPath)
        stateCache[oldPath] = view.state.toJSON({ history: historyField });

      const cache = stateCache[path];
      const stateConfig = createStateConfig(typstState, path, fileId);

      if (cache)
        view.setState(
          EditorState.fromJSON(cache, stateConfig, { history: historyField }),
        );
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
      typst(typstState, storageItem, prelude, fileId),
      typstLanguage(typstState),

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
      autocompletion(),
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

// const activeLineBackground = `rgba(${secondaryContainer.r},${secondaryContainer.g},${secondaryContainer.b},0.25)`;
const activeLineBackground = computed(() => {
  const { secondaryContainer } = palette.value;

  return `rgba(${secondaryContainer.r},${secondaryContainer.g},${secondaryContainer.b},0.25)`;
});
</script>

<template>
  <div class="size-full overflow-x-auto overflow-y-scroll">
    <div ref="container" class="editor" />
  </div>
</template>

<style lang="scss">
.editor {
  .cm-editor {
    @apply m3-body-large h-full outline-none;
  }

  .cm-line {
    @apply p-0 px-[1px] text-[16px];

    font-family: "Iosevka Book";
  }

  .cm-content {
    @apply caret-m3-primary p-0;

    font-family: "Iosevka Book";
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
    background-color: v-bind(activeLineBackground);
  }

  .cm-lintPoint-error::after {
    @apply border-b-m3-error;
  }

  .cm-lintRange {
    @apply decoration-underline bg-none decoration-wavy;
  }

  .cm-lintRange-error {
    @apply decoration-m3-error;
  }

  /* .cm-lintRange-hint {
    @apply decoration-m3-error;
  } */

  .cm-tooltip {
    @apply bg-m3-surface-container-lowest m-0 max-w-xl rounded-lg border-none p-0 shadow;

    font-family: "Iosevka Book";

    [aria-selected="true"] {
      @apply bg-m3-secondary-container! text-m3-on-secondary-container!;
    }

    li {
      @apply p-1!;
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
    @apply text-m3-error border-m3-error;
  }

  .cm-diagnostic-hint {
    @apply text-m3-outline border-m3-outline;
  }

  .typst-render {
    display: inline;
    // vertical-align: bottom;
    cursor: text;
    -wekkit-user-drag: none;
    -moz-user-drag: none;
    // user-select: none;
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
