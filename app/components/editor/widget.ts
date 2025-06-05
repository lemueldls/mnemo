import {
  Decoration,
  ViewPlugin,
  WidgetType,
  EditorView,
} from "@codemirror/view";

import { StateEffect, StateField } from "@codemirror/state";

import type { TypstState, FileId, RenderedFrame } from "mnemo-wasm";

import type { ViewUpdate, DecorationSet } from "@codemirror/view";

import type { Range } from "@codemirror/state";
import { setDiagnostics, type Diagnostic } from "@codemirror/lint";

class TypstWidget extends WidgetType {
  #image = document.createElement("img");

  public constructor(
    private readonly typstState: TypstState,
    private readonly view: EditorView,
    private readonly range: { start: number; end: number },
    private readonly frame: RenderedFrame,
  ) {
    super();

    this.#image.draggable = false;
    this.#image.classList.add("typst-render");
    // this.#image.addEventListener("click", this.handleJump.bind(this));
    // this.#image.addEventListener("mousedown", this.handleJump.bind(this));
  }

  private async handleJump(event: MouseEvent) {
    event.preventDefault();

    const { typstState, range, view } = this;
    const { top, left } = this.#image.getBoundingClientRect();

    const x = event.clientX - left;
    const y = event.clientY - top;

    const jump = typstState.click(0, x, y); // can crash
    const position = jump ? jump.position : range.end;

    view.dispatch({ selection: { anchor: position } });
  }

  public override eq(other: TypstWidget) {
    return (
      other.frame.height === this.frame.height &&
      other.frame.render === this.frame.render
    );
  }

  public toDOM() {
    this.#image.src = `data:image/png;base64,${this.frame.render}`;

    return this.#image;
  }

  public override get estimatedHeight() {
    return (
      this.#image.height || this.frame.height / 2 /* window.devicePixelRatio */
    );
  }

  public override ignoreEvent(event: Event) {
    return event.type === "mousedown";
  }

  public override destroy() {
    this.#image.removeEventListener("click", this.handleJump);
  }
}

function decorate(
  typstState: TypstState,
  update: ViewUpdate,
  fileId: FileId,
  text: string,
  prelude: string,
) {
  const { view, state } = update;

  const syncResult = syncTypstState(typstState, fileId, text, prelude);

  const widgets: Range<Decoration>[] = [];

  function normalizeDiagnostic(diagnostic: Diagnostic) {
    if (diagnostic.from > text.length) diagnostic.from = 0;
    if (diagnostic.to > text.length) diagnostic.to = text.length;

    return diagnostic;
  }

  const diagnostics = syncResult.diagnostics.flatMap((diagnostic) => {
    const diagnostics: Diagnostic[] = [
      normalizeDiagnostic({
        from: diagnostic.range.start,
        to: diagnostic.range.end,
        severity: diagnostic.severity,
        message: diagnostic.message,
      }),
    ];

    for (const hint of diagnostic.hints) {
      diagnostics.push(
        normalizeDiagnostic({
          from: diagnostic.range.start,
          to: diagnostic.range.end,
          severity: "hint",
          message: hint,
        }),
      );
    }

    return diagnostics;
  });

  const transaction = setDiagnostics(state, diagnostics);
  queueMicrotask(() => view.dispatch(transaction));

  for (const { range, render } of syncResult.renders) {
    if (render) {
      const { from: start, number: startLine } = state.doc.lineAt(range.start);
      const { to: end, number: endLine } = state.doc.lineAt(range.end);

      const inactive =
        !view.hasFocus ||
        state.selection.ranges.every(
          (range) =>
            (range.from < start || range.from > end) &&
            (range.to < start || range.to > end) &&
            (start < range.from || start > range.to) &&
            (end < range.from || end > range.to),
        );

      if (inactive)
        widgets.push(
          Decoration.replace({
            widget: new TypstWidget(typstState, view, range, render),
            // inclusive: true,
          }).range(start, end),
        );
      else {
        let lineHeight = 0;

        for (
          let currentLine = startLine;
          currentLine <= endLine;
          currentLine++
        ) {
          const line = state.doc.line(currentLine);

          let style = "";
          if (currentLine == startLine)
            style +=
              "border-top-left-radius:0.25rem;border-top-right-radius:0.25rem;";
          if (currentLine == endLine)
            style += `border-bottom-left-radius:0.25rem;border-bottom-right-radius:0.25rem;min-height:${render.height - lineHeight}px`;
          else lineHeight += view.lineBlockAt(line.from).height;

          widgets.push(
            Decoration.line({
              class: "cm-activeLine",
              attributes: { style },
              // inclusive: true,
            }).range(line.from),
          );
        }
      }
    }
  }

  return Decoration.set(widgets);
}

const stateEffect = StateEffect.define<{ decorations: DecorationSet }>({});

export const viewPlugin = (
  typstState: TypstState,
  item: Ref<Ref<string>>,
  prelude: Ref<string>,
  fileId: FileId,
) =>
  ViewPlugin.define((_view) => {
    return {
      update(update: ViewUpdate) {
        if (
          update.docChanged ||
          update.geometryChanged ||
          update.selectionSet
        ) {
          typstState.resize(
            update.view.contentDOM.clientWidth -
              2 * 2 /* window.devicePixelRatio */,
          );

          const text = update.state.doc.toString();
          item.value.value = text;

          const decorations = decorate(
            typstState,
            update,
            fileId,
            text,
            prelude.value,
          );
          const effects = stateEffect.of({ decorations });

          queueMicrotask(() => update.view.dispatch({ effects }));
        }
      },
    };
  });

export const typst = (
  typstState: TypstState,
  item: Ref<Ref<string>>,
  prelude: Ref<string>,
  fileId: FileId,
) =>
  StateField.define({
    create() {
      return Decoration.none;
    },
    update(decorations, transaction) {
      const effect = transaction.effects.find((effect) =>
        effect.is(stateEffect),
      );

      if (effect) {
        if (effect.value.decorations.size > 0) return effect.value.decorations;

        const max = Math.max(
          ...transaction.state.selection.ranges.map(({ to }) => to),
        );

        return decorations.update({
          filter(from) {
            return from < max;
          },
        });
      }

      return decorations;
    },
    provide: (field) => [
      EditorView.decorations.from(field, (decorations) => decorations),
      viewPlugin(typstState, item, prelude, fileId),
    ],
  });

function syncTypstState(
  typstState: TypstState,
  fileId: FileId,
  text: string,
  prelude: string,
) {
  let result;

  try {
    result = typstState.sync(fileId, text, prelude);
  } catch (error) {
    console.error("LMFAO");
    console.error(error);
    // console.log("retrying...");

    // result = syncTypstState(typstState, fileId, text);

    // window.location.reload();

    throw error;
  }

  return result;
}
