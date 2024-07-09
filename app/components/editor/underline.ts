import { EditorView, Decoration, keymap } from "@codemirror/view";
import { StateField, StateEffect } from "@codemirror/state";

import type { DecorationSet } from "@codemirror/view";

const addUnderline = StateEffect.define<{ from: number; to: number }>({
  map: ({ from, to }, change) => ({
    from: change.mapPos(from),
    to: change.mapPos(to),
  }),
});

const underlineField = StateField.define<DecorationSet>({
  create() {
    return Decoration.none;
  },
  update(underlines, tr) {
    // eslint-disable-next-line unicorn/no-array-callback-reference
    underlines = underlines.map(tr.changes);

    for (const effect of tr.effects)
      if (effect.is(addUnderline))
        underlines = underlines.update({
          add: [underlineMark.range(effect.value.from, effect.value.to)],
        });

    return underlines;
  },
  provide: (f) => EditorView.decorations.from(f),
});

const underlineMark = Decoration.mark({ class: "cm-underline" });

const underlineTheme = EditorView.baseTheme({
  ".cm-underline": {
    textDecoration: "underline",
    textDecorationThickness: "3px",
    textDecorationColor: "red",
  },
});

export function underlineSelection(view: EditorView) {
  const effects: StateEffect<unknown>[] = view.state.selection.ranges
    .filter((r) => !r.empty)
    .map(({ from, to }) => addUnderline.of({ from, to }));

  if (effects.length === 0) return false;

  if (!view.state.field(underlineField, false))
    effects.push(StateEffect.appendConfig.of([underlineField, underlineTheme]));

  view.dispatch({ effects });

  return true;
}

export const underlineKeymap = keymap.of([
  { key: "Mod-u", preventDefault: true, run: underlineSelection },
]);
