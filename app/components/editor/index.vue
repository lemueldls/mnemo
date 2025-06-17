<script setup lang="ts">
import {
  highlightSpecialChars,
  drawSelection,
  dropCursor,
  rectangularSelection,
  crosshairCursor,
  keymap,
  placeholder,
  EditorView,
} from "@codemirror/view";

import { EditorState } from "@codemirror/state";
import {
  // foldGutter,
  indentOnInput,
  syntaxHighlighting,
  defaultHighlightStyle,
  bracketMatching,
} from "@codemirror/language";
import { history, historyField } from "@codemirror/commands";
import { highlightSelectionMatches } from "@codemirror/search";
import { closeBrackets, autocompletion } from "@codemirror/autocomplete";

import { vscodeKeymap } from "@replit/codemirror-vscode-keymap";

import { Rgb } from "mnemo-wasm";

import type { NoteKind } from "~/composables/spaces";

import { typst } from "./widget";

import { typstLanguage } from "./languague";

import type { Rgba } from "@material/material-color-utilities";

import type { EditorStateConfig } from "@codemirror/state";
import { ThemeColors, type TypstState, type FileId } from "mnemo-wasm";

const props = defineProps<{
  spaceId: string;
  kind: NoteKind;
  readonly?: boolean;
}>();

const path = defineModel<string>({ required: true });

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

const text = await useStorageItem(
  () => `spaces/${props.spaceId}/${props.kind}/${path.value}.typ`,
  "",
);

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

  watchImmediate(
    [path, () => props.spaceId, () => props.kind],
    ([path], [oldPath]) =>
      watch(
        text,
        async (text) => {
          const fileId = typstState.insertFile(path, text);

          if (oldPath)
            stateCache[oldPath] = view.state.toJSON({ history: historyField });

          const cache = stateCache[path];
          const stateConfig = await createStateConfig(typstState, path, fileId);

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

async function createStateConfig(
  typstState: TypstState,
  path: string,
  fileId: FileId,
): Promise<EditorStateConfig> {
  const text = await useStorageItem(
    () => `spaces/${props.spaceId}/${props.kind}/${path}.typ`,
    "",
  );

  return {
    extensions: [
      typst(typstState, text, prelude, fileId),
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
      keymap.of(vscodeKeymap),
    ],
  };
}

const activeLineBackground = computed(() => {
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

  .cm-selectionBackground {
    @apply text-on-surface-variant;

    background: var(--md-sys-color-surface-container-highest) !important;
  }

  .cm-selectionMatch {
    @apply bg-tertiary-container text-on-tertiary-container;
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
    display: inline;
    cursor: text;
    vertical-align: top;
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
}
</style>
