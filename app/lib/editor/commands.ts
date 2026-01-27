import { startCompletion } from "@codemirror/autocomplete";
import { indentMore } from "@codemirror/commands";
import {
  ChangeSet,
  EditorSelection,
  EditorState,
  countColumn,
  findColumn,
} from "@codemirror/state";

import type { SelectionRange } from "@codemirror/state";
import type { EditorView } from "@codemirror/view";

export function toggleStrong(view: EditorView) {
  toggleAroundSelection(view, ["*"], ["*"]);

  return true;
}

export function toggleEmph(view: EditorView) {
  toggleAroundSelection(view, ["_"], ["_"]);

  return true;
}

export function toggleUnderline(view: EditorView) {
  toggleAroundSelection(view, ["#underline["], ["]"]);

  return true;
}

export function toggleMath(view: EditorView) {
  toggleAroundSelection(view, ["$", "$ "], ["$", " $"]);

  return true;
}

export function insertTabLike(view: EditorView) {
  return view.state.selection.ranges.some((range) => !range.empty)
    ? indentMore(view)
    : (view.dispatch(
        view.state.changeByRange(({ from: pos }) => {
          const line = view.state.doc.lineAt(pos),
            spaces = countColumn(line.text, 2, pos - line.from) % 2 == 0 ? "  " : " ";

          return {
            changes: {
              from: pos,
              insert: spaces,
            },
            range: EditorSelection.cursor(pos + spaces.length),
          };
        }),
      ),
      true);
}

export function toggleCode(view: EditorView) {
  return (
    toggleAroundSelection(
      view,
      [
        "`",
        {
          regex: /(`{3,}([a-z0-9][a-z0-9-]*)?(\s*)*)/i,
          replacement: "```\n",
        },
      ],
      [
        "`",
        {
          regex: /((\s*)*`{3,})/,
          replacement: "\n```",
        },
      ],
      true,
    ) === 0 &&
      view.state.selection.ranges.every((range) => range.empty) &&
      startCompletion(view),
    true
  );
}

export function insertFontFamily(n: EditorView, font: string) {
  return (wrapOrInsert(n, `#text(font: "${font}")[`, "]", `#set text(font: "${font}")\n`), true);
}

export function cycleHeading(view: EditorView) {
  if (view.state.facet(EditorState.readOnly)) return true;
  const { state } = view;

  return (
    view.dispatch(
      state.changeByRange((r) => {
        let o = r.from;
        let c = r.to;

        const u = state.doc.lineAt(o),
          d = u.from;

        if (r.empty) {
          const f = state.doc.sliceString(u.from, o).match(/^\s*/);
          if (f) o = u.from + f[0].length;
          else {
            o = u.from;
            c = u.to;
          }
        }

        const m = state.sliceDoc(d, c).match(/^(\s*)(=+)/);
        if (m) {
          if (!r.empty) {
            const g = state.doc.sliceString(u.from, o).match(/^\s*/);
            if (g) o = u.from + g[0].length;
            else o = u.from;
          }

          const f = m[2]!.length;

          return f < 6
            ? {
                range: EditorSelection.range(r.from + 1, r.to + 1),
                changes: [
                  {
                    from: o + f,
                    insert: "=",
                  },
                ],
              }
            : {
                range: EditorSelection.range(r.from - f - 1, r.to - f - 1),
                changes: [
                  {
                    from: o,
                    to: o + f + 1,
                    insert: "",
                  },
                ],
              };
        }
        const f = state.doc.lineAt(o);
        let g;
        const v = f.from === 0 ? undefined : state.doc.lineAt(f.from - 1),
          y = v === undefined || state.sliceDoc(v.from, v.to).trim().length === 0;
        if (f.from === o || state.sliceDoc(f.from, o).trim().length === 0) g = y ? 0 : 1;
        else {
          g = 2;
          if (!y) g++;
        }
        const E = f.text.match(/^\s*/)?.[0] ?? "",
          N = state.sliceDoc(o, c).split("\n"),
          _ = N.length > 1 && !N[0]!.startsWith("#[") && !N[N.length - 1]!.endsWith("]"),
          T = (g > 0 ? "\n".repeat(g) + E : "") + "= ",
          M = E + "  ";
        let O = N.map((U, J) =>
          J === 0
            ? _
              ? "#[\n" + E + "  " + U
              : U
            : U.trim() != "]" && !U.startsWith(M)
              ? M + U
              : U,
        ).join("\n");
        if (_) O += "\n" + E + "]";

        let W = T + O;
        const P = state.doc.length === c ? undefined : state.doc.lineAt(c + 1);

        if (!P || P.from >= c)
          if (P && state.sliceDoc(P.from, P.to).match(/^\s*$/) === null) W += "\n";

        return {
          range: r.empty
            ? EditorSelection.cursor(o + T.length + O.length)
            : EditorSelection.range(o + T.length, o + T.length + O.length),
          changes: [
            {
              from: o,
              to: c,
              insert: W,
            },
          ],
        };
      }),
    ),
    view.focus(),
    true
  );
}

export function insertAtCursor(n: EditorView, e: string) {
  const r = n.state.selection.main.head;
  n.dispatch({
    selection: EditorSelection.cursor(r + e.length),
    changes: ChangeSet.of(
      {
        from: r,
        insert: e,
      },
      n.state.doc.length,
    ),
  });
}

export function wrapOrInsert(view: EditorView, prefix: string, suffix: string, insert: string) {
  if (view.state.facet(EditorState.readOnly)) return;
  const c = view.state;
  if (c.selection.main.empty) {
    view.dispatch(
      c.changeByRange((u) => ({
        range: EditorSelection.cursor(u.from + insert.length),
        changes: [
          {
            from: u.from,
            insert: insert,
          },
        ],
      })),
    );
  } else toggleAroundSelection(view, [prefix], [suffix]);
}

interface Replacement {
  regex: RegExp;
  replacement: string;
}

export function toggleAroundSelection(
  view: EditorView,
  prefixes: (string | Replacement)[],
  suffixes: (string | Replacement)[],
  preferNextLevel?: boolean,
) {
  if (view.state.facet(EditorState.readOnly)) return null;
  const { state } = view,
    { doc } = state;
  let matchIndex = null;

  return (
    view.dispatch(
      state.changeByRange((range) => {
        let selRange = range;
        if (range.empty) {
          const line = doc.lineAt(range.from),
            wordMatches = line.text.matchAll(/[\p{L},.]+/gu);
          // If cursor is within a word, select the whole word
          for (const match of wordMatches) {
            const wordStart = line.from + (match.index ?? 0),
              wordEnd = wordStart + match[0].length;
            if (wordStart <= range.from && range.from <= wordEnd)
              selRange = EditorSelection.range(wordStart, wordEnd);
          }
        }

        interface MatchResult {
          before: string;
          after: string;
          match: boolean;
        }

        const matchResults: MatchResult[] = [];
        // Check each prefix-suffix pair
        for (let i = 0; i < prefixes.length; i++) {
          const prefix = prefixes[i]!,
            suffix = suffixes[i]!;
          let isMatch,
            beforeText = "",
            afterText = "";
          const CONTEXT_SIZE = 50;
          let beforeOffset = 0,
            afterOffset = 0;

          // Check prefix
          if (typeof prefix === "string") {
            beforeText = state.sliceDoc(Math.max(selRange.from - prefix.length, 0), selRange.from);
            isMatch = beforeText === prefix;
          } else {
            const beforeContext = state.sliceDoc(
              Math.max(selRange.from - CONTEXT_SIZE, 0),
              selRange.from,
            );
            const prefixMatch = beforeContext.match(prefix.regex);
            if (prefixMatch) {
              beforeText = prefixMatch[1]!;
              beforeOffset =
                beforeContext.length - (beforeText.length + beforeContext.indexOf(beforeText));
              isMatch = true;
            } else isMatch = false;
          }

          // Check suffix
          if (typeof suffix === "string") {
            afterText = state.sliceDoc(
              selRange.to,
              Math.min(selRange.to + suffix.length, doc.length),
            );
            isMatch = isMatch && afterText === suffix;
          } else {
            const afterContext = state.sliceDoc(
              selRange.to,
              Math.min(selRange.to + CONTEXT_SIZE, doc.length),
            );
            const suffixMatch = afterContext.match(suffix.regex);
            if (suffixMatch) {
              afterText = suffixMatch[1]!;
              afterOffset = afterContext.indexOf(afterText);
              isMatch = isMatch && true;
            }
          }

          // Update selection range if needed for regex matches
          if (isMatch && (beforeOffset > 0 || afterOffset > 0)) {
            selRange = EditorSelection.range(
              selRange.from - beforeOffset,
              selRange.to + afterOffset,
            );
          }

          matchResults.push({
            before: beforeText,
            after: afterText,
            match: isMatch,
          });
        }

        // Process match results
        for (let i = 0; i < matchResults.length; i++) {
          if (
            matchResults[i]!.match &&
            preferNextLevel &&
            matchResults.slice(i + 1).some((result) => result.match)
          )
            continue;

          const { before, after, match } = matchResults[i]!;
          if (match) {
            matchIndex = i;
            if (i + 1 < prefixes.length) {
              const nextPrefix = prefixes[i + 1]!,
                nextSuffix = suffixes[i + 1]!,
                newPrefix = typeof nextPrefix === "string" ? nextPrefix : nextPrefix.replacement,
                newSuffix = typeof nextSuffix === "string" ? nextSuffix : nextSuffix.replacement,
                prefixOffset = newPrefix.length - before.length;

              return {
                range:
                  preferNextLevel && range.empty && i === 0
                    ? EditorSelection.cursor(selRange.from + prefixOffset - 1)
                    : EditorSelection.range(
                        selRange.from + prefixOffset,
                        selRange.to + prefixOffset,
                      ),
                changes: [
                  {
                    from: selRange.from - before.length,
                    to: selRange.from,
                    insert: newPrefix,
                  },
                  {
                    from: selRange.to,
                    to: selRange.to + before.length,
                    insert: newSuffix,
                  },
                ],
              };
            }
            // Remove existing delimiters
            return {
              range: EditorSelection.range(
                selRange.from - before.length,
                selRange.to - before.length,
              ),
              changes: [
                {
                  from: selRange.from - before.length,
                  to: selRange.from,
                  insert: "",
                },
                {
                  from: selRange.to,
                  to: selRange.to + after.length,
                  insert: "",
                },
              ],
            };
          }
        }

        // Apply first level of delimiters if no matches
        const firstPrefix = prefixes[0]!,
          firstSuffix = suffixes[0]!,
          prefixText = typeof firstPrefix === "string" ? firstPrefix : firstPrefix.replacement,
          suffixText = typeof firstSuffix === "string" ? firstSuffix : firstSuffix.replacement;

        return {
          range: EditorSelection.range(
            selRange.from + prefixText.length,
            selRange.to + prefixText.length,
          ),
          changes: [
            {
              from: selRange.from,
              insert: prefixText,
            },
            {
              from: selRange.to,
              insert: suffixText,
            },
          ],
        };
      }),
    ),
    // Reset match index if multiple selections
    state.selection.ranges.length > 1 && (matchIndex = null),
    view.focus(),
    matchIndex
  );
}

export function toggleList(n: EditorView) {
  toggleListLike(n, "- ");

  return true;
}

export function toggleEnum(n: EditorView) {
  toggleListLike(n, "+ ");

  return true;
}

export function toggleListLike(view: EditorView, prefix: string) {
  if (view.state.facet(EditorState.readOnly)) return;
  const { doc } = view.state;

  view.dispatch(
    view.state.changeByRange((range) => {
      const changes: { from: number; to: number; insert: string }[] = [];
      let fromOffset = 0,
        toOffset = 0;

      const calculateOffset = (pos: number, change: { from: number; to: number; insert: string }) =>
        change.from <= pos ? change.insert.length - (Math.min(change.to, pos) - change.from) : 0;

      const addChange = (change: { from: number; to: number; insert: string }) => {
        changes.push(change);
        fromOffset += calculateOffset(range.from, change);
        toOffset += calculateOffset(range.to, change);
      };

      const getLineStart = (lineNum: number) => {
        const line = doc.line(lineNum);
        const indentMatch = line.text.match(/^\s*/);
        return line.from + (indentMatch?.[0].length ?? 0);
      };

      const startLine = doc.lineAt(range.from),
        endLine = doc.lineAt(range.to),
        listMarkers: [number, number][] = [];

      let currentLineNum = startLine.number,
        hasListMarkers = false;

      const lineIter = doc.iterLines(startLine.number, endLine.number + 1);
      while (lineIter.next() && !lineIter.done) {
        const listMatch = lineIter.value.trimStart().match(/^(-|\+|\d*\.)\s?/);
        if (listMatch !== null) {
          listMarkers.push([currentLineNum, listMatch[0].length]);
          hasListMarkers = true;
        }
        currentLineNum++;
      }

      if (hasListMarkers) {
        // Remove existing list markers
        for (const [lineNum, markerLength] of listMarkers) {
          const lineStart = getLineStart(lineNum);
          addChange({
            from: lineStart,
            to: lineStart + markerLength,
            insert: "",
          });
        }
      } else {
        // Add list markers
        for (let lineNum = startLine.number; lineNum <= endLine.number; lineNum++) {
          const lineStart = getLineStart(lineNum);
          addChange({
            from: lineStart,
            to: lineStart,
            insert: prefix,
          });
        }
      }

      return {
        range: EditorSelection.range(range.from + fromOffset, range.to + toOffset),
        changes,
      };
    }),
  );

  view.focus();
}

export function cursorAddVertical(view: EditorView, offset: number) {
  const { state } = view,
    newCursors: SelectionRange[] = [];

  for (const range of state.selection.ranges) {
    const currentLine = state.doc.lineAt(range.from),
      currentColumn = countColumn(currentLine.text, state.tabSize, range.from - currentLine.from),
      targetLineNum = currentLine.number + offset;

    if (targetLineNum >= 1 && targetLineNum <= state.doc.lines) {
      const targetLine = state.doc.line(currentLine.number + offset),
        targetPos = targetLine.from + findColumn(targetLine.text, currentColumn, state.tabSize);
      newCursors.push(EditorSelection.cursor(targetPos));
    }
  }

  if (offset > 0) newCursors.reverse();

  newCursors.push(...state.selection.ranges);

  view.dispatch({
    selection: EditorSelection.create(newCursors),
    scrollIntoView: true,
    userEvent: "select",
  });

  return true;
}
