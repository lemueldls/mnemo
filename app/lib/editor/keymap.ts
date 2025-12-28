import { acceptCompletion, closeBracketsKeymap, completionKeymap } from "@codemirror/autocomplete";

import {
  copyLineDown,
  copyLineUp,
  indentLess,
  insertBlankLine,
  moveLineDown,
  moveLineUp,
  redo,
  redoSelection,
  simplifySelection,
  standardKeymap,
  toggleBlockComment,
  toggleLineComment,
  undo,
  undoSelection,
} from "@codemirror/commands";

import { nextDiagnostic } from "@codemirror/lint";

import {
  closeSearchPanel,
  findNext,
  findPrevious,
  gotoLine,
  openSearchPanel,
  selectNextOccurrence,
  selectSelectionMatches,
} from "@codemirror/search";

import { keymap } from "@codemirror/view";

import {
  cursorAddVertical,
  insertTabLike,
  toggleEmph,
  toggleStrong,
  toggleUnderline,
} from "./commands";

export const typstKeymap = keymap.of([
  ...completionKeymap,
  ...closeBracketsKeymap,
  {
    key: "Tab",
    run: acceptCompletion,
  },
  {
    key: "Mod-z",
    run: undo,
    preventDefault: true,
  },
  {
    key: "Mod-y",
    mac: "Mod-Shift-z",
    run: redo,
    preventDefault: true,
  },
  {
    key: "Mod-Shift-z",
    run: redo,
    preventDefault: true,
  },
  {
    key: "Mod-u",
    run: undoSelection,
    preventDefault: true,
  },
  {
    key: "Alt-u",
    mac: "Mod-Shift-u",
    run: redoSelection,
    preventDefault: true,
  },
  {
    key: "Mod-/",
    run: toggleLineComment,
  },
  {
    mac: "Cmd-Alt-/",
    win: "Shift-Alt-a",
    linux: "Ctrl+Shift+a",
    run: toggleBlockComment,
  },
  {
    key: "Mod-#",
    run: toggleLineComment,
  },
  {
    key: "Mod-Shift-7",
    run: toggleLineComment,
  },
  {
    key: "Mod-Shift-/",
    run: toggleLineComment,
  },
  {
    key: "Shift-Alt-a",
    run: toggleBlockComment,
  },
  {
    key: "Mod-b",
    run: toggleStrong,
  },
  {
    key: "Mod-Shift-f",
    run: toggleStrong,
  },
  {
    key: "Mod-i",
    run: toggleEmph,
  },
  {
    key: "Mod-Shift-K",
    run: toggleEmph,
  },
  {
    key: "Mod-u",
    run: toggleUnderline,
  },
  {
    key: "Alt-ArrowUp",
    run: moveLineUp,
  },
  {
    key: "Shift-Alt-ArrowUp",
    run: copyLineUp,
  },
  {
    key: "Alt-ArrowDown",
    run: moveLineDown,
  },
  {
    key: "Shift-Alt-ArrowDown",
    run: copyLineDown,
  },
  {
    key: "Tab",
    run: insertTabLike,
    shift: indentLess,
  },
  {
    key: "Mod-Enter",
    run: insertBlankLine,
  },
  {
    key: "F8",
    run: nextDiagnostic,
  },
  {
    key: "Mod-g",
    run: gotoLine,
    scope: "editor goto-panel search-panel",
  },
  {
    key: "Escape",
    run: simplifySelection,
  },
  {
    key: "Mod-Alt-ArrowUp",
    run: (n) => cursorAddVertical(n, -1),
  },
  {
    key: "Mod-Alt-ArrowDown",
    run: (n) => cursorAddVertical(n, 1),
  },
  {
    key: "Mod-f",
    run: openSearchPanel,
    scope: "editor search-panel goto-panel",
  },
  {
    key: "F3",
    run: findNext,
    shift: findPrevious,
    scope: "editor search-panel",
    preventDefault: true,
  },
  {
    key: "Escape",
    run: closeSearchPanel,
    scope: "editor search-panel",
  },
  {
    key: "Mod-Shift-l",
    run: selectSelectionMatches,
  },
  {
    key: "Mod-d",
    run: selectNextOccurrence,
    preventDefault: true,
  },
  ...standardKeymap,
]);
