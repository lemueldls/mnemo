import { LanguageSupport, LRLanguage, foldable } from "@codemirror/language";
import { invoke } from "@tauri-apps/api/core";

import { parser } from "./parser";

import type {
  CompletionContext,
  CompletionResult,
  Completion,
} from "@codemirror/autocomplete";

export const language = LRLanguage.define({
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
    // autocomplete,
  },
});

async function autocomplete(
  context: CompletionContext,
): Promise<CompletionResult> {
  const { pos, explicit } = context;

  const [offset, completions] = await invoke<[number, Completion[]]>(
    "typst_autocomplete",
    { cursor: pos, explicit },
  );

  // console.log({ offset, completions });

  return { from: offset, options: completions };
}

// const fold = foldable()

export function typstLanguage() {
  return new LanguageSupport(language);
}
