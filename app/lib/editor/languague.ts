import { LanguageSupport, LRLanguage } from "@codemirror/language";

import { parser } from "./parser";

import type {
  CompletionContext,
  CompletionResult,
} from "@codemirror/autocomplete";

import type { TypstState } from "mnemo-wasm";

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
      return {
        type: completion.type,
        label: completion.label,
        apply: completion.apply,
        info: completion.detail,
      };
    }),
  };
}

export function typstLanguage(typstState: TypstState) {
  return new LanguageSupport(createLanguage(typstState));
}
