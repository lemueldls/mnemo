import { setDiagnostics, type Diagnostic } from "@codemirror/lint";
import { StateEffect, StateField } from "@codemirror/state";

import {
  Decoration,
  EditorView,
  ViewPlugin,
  WidgetType,
} from "@codemirror/view";

import { LRUCache } from "lru-cache";

import type { Range } from "@codemirror/state";
import type { DecorationSet, ViewUpdate } from "@codemirror/view";

import type {
  CompileResult,
  FileId,
  RangedFrame,
  TypstState,
} from "mnemo-wasm";

class TypstWidget extends WidgetType {
  #container = document.createElement("div");
  #image = document.createElement("img");

  public constructor(
    private readonly typstState: TypstState,
    private readonly view: EditorView,
    private readonly frame: RangedFrame,
  ) {
    super();

    this.#container.classList.add("typst-render");
    this.#container.style.height = `${frame.render.height}px`;

    this.#image.draggable = false;
    this.#image.src = `data:image/png;base64,${this.frame.render.encoding}`;
    this.#image.addEventListener("click", this.handleMouseEvent.bind(this));
    this.#image.addEventListener("mousedown", this.handleMouseEvent.bind(this));
    this.#image.addEventListener(
      "touchstart",
      this.handleTouchEvent.bind(this),
    );

    this.#container.append(this.#image);
  }

  private handleMouseEvent(event: MouseEvent) {
    event.preventDefault();
    const { clientX, clientY } = event;
    this.handleJump(clientX, clientY);
  }

  private handleTouchEvent(event: TouchEvent) {
    event.preventDefault();
    const [touch] = event.touches;
    const { clientX, clientY } = touch!;
    this.handleJump(clientX, clientY);
  }

  private async handleJump(clientX: number, clientY: number) {
    const { typstState, frame, view } = this;
    const { top, left } = this.#image.getBoundingClientRect();

    const x = clientX - left;
    const y = clientY - top;

    const jump = typstState.click(x, y + frame.render.offsetHeight);
    const position = jump ? jump.position : frame.range.end;

    view.focus();
    view.dispatch({ selection: { anchor: position } });
  }

  public override eq(other: TypstWidget) {
    return (
      other.frame.render.height === this.frame.render.height &&
      other.frame.render.encoding === this.frame.render.encoding
    );
  }

  public toDOM() {
    return this.#container;
  }

  public override get estimatedHeight() {
    return this.frame.render.height;
  }

  public override destroy() {
    this.#image.removeEventListener("click", this.handleMouseEvent.bind(this));
    this.#image.removeEventListener(
      "mousedown",
      this.handleMouseEvent.bind(this),
    );
    this.#image.removeEventListener(
      "touchstart",
      this.handleTouchEvent.bind(this),
    );
  }
}

const cache = new LRUCache<string, CompileResult>({ max: 3 });

function decorate(
  typstState: TypstState,
  update: ViewUpdate,
  path: string,
  fileId: FileId,

  prelude: string,
) {
  const text = update.state.doc.toString();

  let compileResult: CompileResult;
  if (update.docChanged || !cache.has(path)) {
    compileResult = typstState.compile(fileId, text, prelude);
    cache.set(path, compileResult);
  } else compileResult = cache.get(path)!;

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
  path: string,
  fileId: FileId,
  prelude: Ref<string>,
) =>
  ViewPlugin.define((_view) => {
    return {
      update(update: ViewUpdate) {
        if (
          update.docChanged ||
          update.geometryChanged ||
          update.selectionSet ||
          update.focusChanged
        ) {
          typstState.resize(
            update.view.contentDOM.clientWidth - 2 * window.devicePixelRatio,
          );

          const decorations = decorate(
            typstState,
            update,
            path,
            fileId,

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
  path: string,
  fileId: FileId,
  prelude: Ref<string>,
) =>
  StateField.define({
    create() {
      return Decoration.none;
    },
    update(decorations, transaction) {
      const effect = transaction.effects.find((effect) =>
        effect.is(stateEffect),
      );

      if (effect?.value.decorations) {
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
      viewPlugin(typstState, path, fileId, prelude),
    ],
  });
