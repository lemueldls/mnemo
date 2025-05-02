import { LanguageSupport, LRLanguage, foldable } from "@codemirror/language";
import { invoke } from "@tauri-apps/api/core";

import { parser } from "./parser";

import type {
  CompletionContext,
  CompletionResult,
  Completion,
} from "@codemirror/autocomplete";
import type { TypstState } from "~~/backend/wasm/pkg/mnemo_wasm";

export const createLanguage = (typstState) =>
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

  console.log({ context });
  const [offset, completions] = typstState.autocomplete(pos, explicit);
  console.log({ offset, completions });

  return { from: offset, options: completions };
}

// const fold = foldable()

export function typstLanguage(typstState: TypstState) {
  return new LanguageSupport(createLanguage(typstState));
}
