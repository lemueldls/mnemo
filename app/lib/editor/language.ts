import {
  snippet,
  startCompletion,
  type Completion,
  type CompletionContext,
  type CompletionResult,
} from "@codemirror/autocomplete";

import { LanguageSupport, LRLanguage } from "@codemirror/language";

import { parser } from "./parser";

import type { EditorView } from "@codemirror/view";
import type { TypstState } from "mnemo-wasm";

export const createLanguage = (typstState: TypstState) =>
  LRLanguage.define({
    name: "typst",
    parser,
    languageData: {
      closeBrackets: {
        brackets: ["(", "[", "{", '"', "`", "$", "_", "*"],
        before: ')]}"`$,;',
      },
      commentTokens: {
        line: "//",
        block: {
          open: "/*",
          close: "*/",
        },
      },
      wordChars: "-_",
      indentOnInput: /^\s*[[\]{}()]$/,
      autocomplete: (context: CompletionContext) =>
        autocomplete(typstState, context),
    },
  });

async function autocomplete(
  typstState: TypstState,
  context: CompletionContext,
): Promise<CompletionResult | null> {
  const { pos, explicit } = context;
  const result = typstState.autocomplete(pos, explicit);
  if (!result) return null;

  const { offset, completions } = result;

  return {
    from: offset,
    options: completions.map((completion) => {
      const type =
        typeof completion.type == "object" ? "symbol" : completion.type;
      const result: Completion = {
        type,
        apply: completion.apply,
        label: completion.label,
        info: completion.detail,
      };

      if (completion.apply?.includes("$")) {
        const tt =
            completion.apply.match(/\${\w*}/)?.at(0) == "${}" &&
            (completion.type !== "syntax" || completion.label != "linebreak"),
          applySnippet = snippet(completion.apply);

        result.apply = (
          view: EditorView,
          completion: Completion,
          from: number,
          to: number,
        ) => {
          applySnippet(view, completion, from, to);
          if (tt) startCompletion(view);
        };
      }

      return result;
    }),
  };
}

export function typstLanguage(typstState: TypstState) {
  return new LanguageSupport(createLanguage(typstState));
}
