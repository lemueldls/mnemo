import { LanguageSupport, LRLanguage } from "@codemirror/language";

import { parser } from "./parser";

import type {
  CompletionContext,
  CompletionResult,
} from "@codemirror/autocomplete";
import type { TypstState } from "mnemo-wasm";
import { EditorSelection } from "@codemirror/state";

export const createLanguage = (typstState: TypstState) =>
  LRLanguage.define({
    name: "typst",
    parser,
    languageData: {
      closeBrackets: {
        brackets: [
          "(",
          "[",
          "{",
          "'",
          '"',
          "`",
          "```",
          "*",
          "**",
          "_",
          "__",
          "$",
        ],
      },
      commentTokens: { line: "//" },
      autocomplete: (context) => autocomplete(typstState, context),
    },
  });

async function autocomplete(
  typstState: TypstState,
  context: CompletionContext,
): Promise<CompletionResult> {
  const { pos, explicit } = context;
  const { offset, completions } = typstState.autocomplete(pos, explicit);

  return {
    from: offset,
    options: completions.map((completion) => {
      const { apply } = completion;

      return {
        type: completion.type,
        label: completion.label,
        apply(view, _completion, from, to) {
          if (!apply) return;

          const matches = apply.matchAll(/\${(.*)}/gm);
          const filtered = apply.replaceAll(/\${(.*)}/gm, "$1");

          const ranges = [];
          let offset = 0;
          for (const match of matches) {
            const from = match.index;
            const to = from + match[1]!.length;

            ranges.push(EditorSelection.range(from - offset, to - offset));

            offset += 3; // ${}
          }

          if (matches)
            view.dispatch({
              changes: {
                from: from,
                to: to,
                insert: filtered,
              },
              selection: EditorSelection.create(ranges, 1),
            });
        },
        info: completion.detail,
      };
    }),
  };
}

export function typstLanguage(typstState: TypstState) {
  return new LanguageSupport(createLanguage(typstState));
}
