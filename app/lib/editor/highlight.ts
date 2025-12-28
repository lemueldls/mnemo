import { EditorState, RangeSetBuilder, StateField } from "@codemirror/state";
import { Decoration, EditorView, type DecorationSet } from "@codemirror/view";
import { FileId, TypstState } from "mnemo-wasm";

export const typstSyntaxHighlighting = (fileId: FileId, typstState: TypstState) =>
  StateField.define<DecorationSet>({
    create(state) {
      return buildDecorations(state, fileId, typstState);
    },
    update(decoration, transaction) {
      if (transaction.docChanged) return buildDecorations(transaction.state, fileId, typstState);
      return decoration;
    },
    provide(field) {
      return EditorView.decorations.from(field);
    },
  });

function buildDecorations(
  state: EditorState,
  fileId: FileId,
  typstState: TypstState,
): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  const tokens = typstState.highlight(fileId, state.doc.toString());

  for (const token of tokens) {
    builder.add(token.range.start, token.range.end, Decoration.mark({ class: token.tag }));
  }

  return builder.finish();
}

export const parseBackticks = (str: string, into: HTMLElement) => {
  const result = str.split("`").map((sub, i) => {
    if (i % 2) {
      const code = document.createElement("code");
      code.textContent = sub;
      return code;
    } else {
      return document.createTextNode(sub);
    }
  });

  into.append(...result);
};
