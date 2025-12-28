import { FileId, TypstState } from "mnemo-wasm";
import { typstStateField, typstViewPlugin } from "./widgets";
import { autocomplete, typstLanguageData } from "./language";
import { typstSyntaxHighlighting } from "./highlight";
import { autocompletion } from "@codemirror/autocomplete";
import { EditorView } from "@codemirror/view";
import { type Extension } from "@codemirror/state";
import { typstHoverTooltip } from "./hover";
import { IndentContext, indentService } from "@codemirror/language";
import { typstKeymap } from "./keymap";

export const typstPlugin = (
  fileId: FileId,
  spaceId: string,
  path: string,
  text: Ref<string>,
  prelude: Ref<string>,
  locked: boolean,
  typstState: TypstState,
): Extension => [
  typstStateField,
  typstViewPlugin(fileId, spaceId, path, text, prelude, locked, typstState),

  autocompletion({
    override: [(context) => autocomplete(context, fileId, typstState)],
  }),

  typstKeymap,
  typstLanguageData,
  typstSyntaxHighlighting(fileId, typstState),
  typstHoverTooltip(fileId, typstState),

  addSpaceBeforeClosingBracket,
  indentService.of((ctx: IndentContext, pos: number): number => {
    const last = Math.max(0, pos - 1);
    const prev = ctx.lineAt(last).text;
    if (prev.endsWith("$") && prev !== "$") return 0;
    const indent = /[{[($]\s*$/.test(prev);
    return ctx.lineIndent(last) + (indent ? ctx.unit : 0);
  }),
];

const addSpaceBeforeClosingBracket = EditorView.inputHandler.of((view, from, to, text) => {
  if (text === " ") {
    const state = view.state;
    const pos = from;
    const bracketPairs = { "(": ")", "[": "]", "{": "}", $: "$" };
    const before = state.doc.sliceString(pos - 1, pos) as keyof typeof bracketPairs;
    const after = state.doc.sliceString(pos, pos + 1) as keyof typeof bracketPairs;

    if (bracketPairs[before] && after === bracketPairs[before]) {
      // Insert a space before the closing bracket
      view.dispatch({
        changes: { from: pos, to: pos, insert: " " },
        selection: { anchor: pos + 1 },
      });
    }
  }

  return false;
});
