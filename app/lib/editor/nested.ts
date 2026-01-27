import { EditorState, Prec, StateField, Transaction } from "@codemirror/state";
import { Decoration, EditorView, keymap, WidgetType } from "@codemirror/view";

// --- 1. Utility: Parse Blocks ---
function findBlocks(docString: string) {
  const blocks = [];
  const regex = /\{\{\{([\s\S]*?)\}\}\}/g;
  let match;
  while ((match = regex.exec(docString)) !== null) {
    blocks.push({
      from: match.index,
      to: match.index + match[0].length,
      content: match[1],
      contentStart: match.index + 3, // skip {{{
      contentEnd: match.index + match[0].length - 3, // skip }}}
    });
  }
  return blocks;
}

// --- 2. Widget: The "Output" Preview ---
class PreviewWidget extends WidgetType {
  constructor(
    public content: string,
    public from: number,
  ) {
    super();
  }

  public toDOM(view: EditorView) {
    const wrap = document.createElement("div");
    wrap.className = "cm-preview-widget";

    const label = document.createElement("span");
    label.className = "cm-preview-label";
    label.textContent = "Document Output (Click to Edit)";
    wrap.appendChild(label);

    const result = document.createElement("div");
    // Mock execution result
    result.textContent = `> Output: ${this.content.length} chars. Hash: ${this.content
      .split("")
      .reduce((a, b) => a + b.charCodeAt(0), 0)
      .toString(16)}`;
    wrap.appendChild(result);

    wrap.addEventListener("click", (e) => {
      e.preventDefault();
      // Move cursor inside to trigger switch to Editor mode
      view.dispatch({
        selection: { anchor: this.from + 3 },
        scrollIntoView: true,
      });
    });

    return wrap;
  }

  public override eq(other: PreviewWidget) {
    return other.content === this.content && other.from === this.from;
  }
}

// --- 3. Widget: The Nested Editor ---
class NestedEditorWidget extends WidgetType {
  dom?: HTMLDivElement;
  nestedEditorView?: EditorView;

  constructor(
    public content: string,
    public from: number,
    public to: number,
  ) {
    super();
  }

  toDOM(parentView: EditorView) {
    const wrapper = document.createElement("div");
    wrapper.className = "cm-nested-widget";

    const header = document.createElement("div");
    header.className = "cm-nested-header";
    header.innerHTML = `<span>Edit Source</span> <span>Ln ${parentView.state.doc.lineAt(this.from).number}</span>`;
    wrapper.appendChild(header);

    const editorContainer = document.createElement("div");
    editorContainer.className = "cm-nested-editor-instance";
    wrapper.appendChild(editorContainer);

    // --- KEYMAP: Seamless Navigation Logic ---
    const escapeKeymap = keymap.of([
      {
        key: "ArrowUp",
        run: (view) => {
          const { state } = view;
          const head = state.selection.main.head;
          const line = state.doc.lineAt(head);

          // If cursor is on the FIRST line of the nested editor
          if (line.number === 1) {
            const dom = this.dom;
            if (!dom) return false;

            const parentPos = parentView.posAtDOM(dom);
            if (parentPos === null) return false;

            // UPDATE: Go to parentPos - 1 (outside the block range) to exit.
            // This moves cursor to the end of the line *before* the widget.
            const targetPos = Math.max(0, parentPos - 1);

            parentView.focus();
            parentView.dispatch({
              selection: { anchor: targetPos, head: targetPos },
              scrollIntoView: true,
            });
            return true;
          }
          return false;
        },
      },
      {
        key: "ArrowDown",
        run: (view) => {
          const { state } = view;
          const head = state.selection.main.head;
          const lastLine = state.doc.lineAt(state.doc.length);
          const currentLine = state.doc.lineAt(head);

          if (currentLine.number === lastLine.number) {
            const dom = this.dom;
            if (!dom) return false;

            const parentPos = parentView.posAtDOM(dom);
            if (parentPos === null) return false;

            const docString = parentView.state.doc.toString();
            const closingIndex = docString.indexOf("}}}", parentPos + 3);
            if (closingIndex === -1) return false;

            const endPos = closingIndex + 3;

            parentView.focus();
            parentView.dispatch({
              selection: { anchor: endPos, head: endPos },
              scrollIntoView: true,
            });
            return true;
          }
          return false;
        },
      },
    ]);

    // Create the inner editor
    const view = new EditorView({
      state: EditorState.create({
        doc: this.content,
        extensions: [
          Prec.highest(escapeKeymap),
          // minimalSetup,
          // javascript(),
          // keymap.of([indentWithTab]),
          EditorView.updateListener.of((update) => {
            if (
              update.docChanged &&
              update.transactions.some(
                (tr) =>
                  tr.isUserEvent("input") ||
                  tr.isUserEvent("delete") ||
                  tr.isUserEvent("undo") ||
                  tr.isUserEvent("redo"),
              )
            ) {
              this.syncToParent(parentView, update.state.doc.toString());
            }
          }),
        ],
      }),
      parent: editorContainer,
    });

    this.dom = wrapper;
    this.nestedEditorView = view;

    // --- FOCUS LOGIC ---
    // Wait for render to settle, then focus inner editor.
    setTimeout(() => {
      view.focus();

      // Smart Cursor Placement:
      const parentHead = parentView.state.selection.main.head;
      if (parentHead >= this.to) {
        // Entering from Bottom
        const endLen = view.state.doc.length;
        view.dispatch({ selection: { anchor: endLen } });
      } else {
        // Entering from Top
        view.dispatch({
          selection: { anchor: 0 },
          // UPDATE: Force scroll to top (y: 'start')
          effects: EditorView.scrollIntoView(0, { y: "start" }),
        });
      }
    }, 0);

    return wrapper;
  }

  syncToParent(parentView: EditorView, newContent: string) {
    const dom = this.dom || parentView.dom.querySelector(".cm-nested-widget");
    if (!dom) return;

    const currentFrom = parentView.posAtDOM(dom);
    if (currentFrom === null || currentFrom === undefined) return;

    const docString = parentView.state.doc.toString();
    const closingIndex = docString.indexOf("}}}", currentFrom + 3);
    if (closingIndex === -1) return;

    const currentTo = closingIndex + 3;
    const currentParentContent = docString.substring(currentFrom + 3, currentTo - 3);

    if (newContent !== currentParentContent) {
      parentView.dispatch({
        changes: {
          from: currentFrom + 3,
          to: currentTo - 3,
          insert: newContent,
        },
        annotations: Transaction.userEvent.of("nested-update"),
      });
    }
  }

  override updateDOM(dom: HTMLDivElement, parentView: EditorView) {
    const view = this.nestedEditorView;
    if (!view) return false;

    const currentChildDoc = view.state.doc.toString();

    if (this.content !== currentChildDoc) {
      view.dispatch({
        changes: {
          from: 0,
          to: currentChildDoc.length,
          insert: this.content,
        },
      });
    }

    const headerSpan = dom.querySelector(".cm-nested-header span:last-child");
    if (headerSpan) {
      headerSpan.textContent = `Ln ${parentView.state.doc.lineAt(this.from).number}`;
    }

    this.dom = dom;
    return true;
  }

  override eq(other: NestedEditorWidget) {
    return other.from === this.from;
  }

  override destroy() {
    this.nestedEditorView?.destroy();
  }
}

// --- 4. StateField: Logic for Replacement ---
const blockHideField = StateField.define({
  create(state) {
    return computeDecorations(state);
  },
  update(decorations, transaction) {
    if (transaction.docChanged || transaction.selection) {
      return computeDecorations(transaction.state);
    }
    return decorations;
  },
  provide: (f) => EditorView.decorations.from(f),
});

function computeDecorations(state: EditorState) {
  const widgets = [];
  const docString = state.doc.toString();
  const blocks = findBlocks(docString);
  const selection = state.selection.main;

  for (const block of blocks) {
    // FIXED: Inclusive cursor check for bounds (>= and <=)
    const isCursorInside = selection.head >= block.from && selection.head <= block.to;

    if (isCursorInside) {
      widgets.push(
        Decoration.replace({
          widget: new NestedEditorWidget(block.content!, block.from, block.to),
          // FIXED: No 'block: true', no 'inclusiveStart/End' (defaults to false/inline)
        }).range(block.from, block.to),
      );
    } else {
      widgets.push(
        Decoration.replace({
          widget: new PreviewWidget(block.content!, block.from),
          // FIXED: No 'block: true', no 'inclusiveStart/End'
        }).range(block.from, block.to),
      );
    }
  }
  return Decoration.set(widgets);
}
