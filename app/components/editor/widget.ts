import {
  Decoration,
  ViewPlugin,
  WidgetType,
  EditorView,
} from "@codemirror/view";

import { StateEffect, StateField } from "@codemirror/state";

import type { TypstState, FileId, RangedFrame } from "mnemo-wasm";

import type { ViewUpdate, DecorationSet } from "@codemirror/view";

import type { Range } from "@codemirror/state";
import { setDiagnostics, type Diagnostic } from "@codemirror/lint";

class TypstWidget extends WidgetType {
  #image = document.createElement("img");

  public constructor(
    private readonly typstState: TypstState,
    private readonly view: EditorView,
    private readonly frame: RangedFrame,
  ) {
    super();

    this.#image.draggable = false;
    this.#image.classList.add("typst-render");
    this.#image.addEventListener("click", this.handleJump.bind(this));
    // this.#image.addEventListener("mousedown", this.handleJump.bind(this));
  }

  private async handleJump(event: MouseEvent) {
    event.preventDefault();

    const { typstState, frame, view } = this;
    const { top, left } = this.#image.getBoundingClientRect();

    const x = event.clientX - left;
    const y = event.clientY - top;

    const jump = typstState.click(x, y + frame.render.offsetHeight);
    const position = jump ? jump.position : frame.range.end;

    view.dispatch({ selection: { anchor: position } });
  }

  public override eq(other: TypstWidget) {
    return (
      other.frame.render.height === this.frame.render.height &&
      other.frame.render.encoding === this.frame.render.encoding
    );
  }

  public toDOM() {
    this.#image.src = `data:image/png;base64,${this.frame.render.encoding}`;

    return this.#image;
  }

  public override get estimatedHeight() {
    return (
      this.#image.height || this.frame.render.height / window.devicePixelRatio
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
  const compileResult = compileTypstState(typstState, fileId, text, prelude);

  const { view, state } = update;

  const widgets: Range<Decoration>[] = [];

  if (compileResult.diagnostics.length > 0) {
    const diagnostics = compileResult.diagnostics.flatMap((diagnostic) => {
      const diagnostics: Diagnostic[] = [
        {
          from: diagnostic.range.start,
          to: diagnostic.range.end,
          severity: diagnostic.severity,
          message: diagnostic.message,
        },
      ];

      for (const hint of diagnostic.hints) {
        diagnostics.push({
          from: diagnostic.range.start,
          to: diagnostic.range.end,
          severity: "hint",
          message: hint,
        });
      }

      return diagnostics;
    });

    const transaction = setDiagnostics(state, diagnostics);
    queueMicrotask(() => view.dispatch(transaction));
  } else queueMicrotask(() => view.dispatch(setDiagnostics(state, [])));

  for (const frame of compileResult.frames) {
    if (frame.render) {
      const { from: start, number: startLine } = state.doc.lineAt(
        frame.range.start,
      );
      const { to: end, number: endLine } = state.doc.lineAt(frame.range.end);

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
            widget: new TypstWidget(typstState, view, frame),
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
            style += `border-bottom-left-radius:0.25rem;border-bottom-right-radius:0.25rem;min-height:${frame.render.height - lineHeight}px`;
          else lineHeight += view.lineBlockAt(line.from).height;

          widgets.push(
            Decoration.line({
              class: "cm-activeLine",
              attributes: { style },
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
  textItem: Ref<string>,
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
            update.view.contentDOM.clientWidth - 2 * window.devicePixelRatio,
          );

          const text = update.state.doc.toString();
          if (update.docChanged) textItem.value = text;

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
  textItem: Ref<string>,
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
      viewPlugin(typstState, textItem, prelude, fileId),
    ],
  });

function compileTypstState(
  typstState: TypstState,
  fileId: FileId,
  text: string,
  prelude: string,
) {
  let result;

  try {
    result = typstState.compile(fileId, text, prelude);
  } catch (error) {
    console.error(error);

    // console.log("retrying...");
    // result = syncTypstState(typstState, fileId, text);
    // window.location.reload();

    throw error;
  }

  return result;
}
