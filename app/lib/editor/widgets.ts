import { setDiagnostics, type Diagnostic } from "@codemirror/lint";
import {
  EditorState,
  StateEffect,
  StateField,
  type Range,
} from "@codemirror/state";

import {
  Decoration,
  EditorView,
  ViewPlugin,
  WidgetType,
} from "@codemirror/view";

import { LRUCache } from "lru-cache";

import type { DecorationSet, ViewUpdate } from "@codemirror/view";

import type {
  CompileResult,
  FileId,
  RangedFrame,
  TypstDiagnostic,
  TypstState,
} from "mnemo-wasm";
// import { parseBackticks } from "./highlight";

const containerCache = new LRUCache<number, HTMLDivElement>({ max: 128 });

class TypstWidget extends WidgetType {
  container = document.createElement("div");

  public constructor(
    private readonly view: EditorView,
    private readonly frame: RangedFrame,
    locked: boolean,
    private readonly fileId: FileId,
    private readonly typstState: TypstState,
  ) {
    super();

    const container = containerCache.get(frame.render.hash);

    if (container && container.isConnected) {
      this.container = container;
    } else {
      this.container.dataset.hash = frame.render.hash.toString();
      this.container.classList.add("typst-render");
      this.container.style.height = `${frame.render.height}px`;

      const image = document.createElement("img");

      image.draggable = false;
      image.src = `data:image/png;base64,${frame.render.encoding.toBase64()}`;
      image.height = frame.render.height;

      if (!locked) {
        this.container.addEventListener(
          "click",
          this.handleMouseEvent.bind(this),
        );
        this.container.addEventListener(
          "mousedown",
          this.handleMouseEvent.bind(this),
        );
        // this.#container.addEventListener(
        //   "touchstart",
        //   this.handleTouchEvent.bind(this),
        // );
      }

      this.container.append(image);

      containerCache.set(frame.render.hash, this.container);
    }
  }

  private handleMouseEvent(event: MouseEvent) {
    event.preventDefault();
    const { clientX, clientY } = event;
    this.handleJump(clientX, clientY);
  }

  // private handleTouchEvent(event: TouchEvent) {
  //   event.preventDefault();
  //   const [touch] = event.touches;
  //   const { clientX, clientY } = touch!;
  //   this.handleJump(clientX, clientY);
  // }

  private async handleJump(clientX: number, clientY: number) {
    const { typstState, frame, view } = this;
    const { top, left } = this.container.getBoundingClientRect();

    const x = clientX - left;
    const y = clientY - top;

    const jump = typstState.click(
      this.fileId,
      x,
      y + frame.render.offsetHeight,
    );
    const position = jump ? jump.position : frame.range.end;

    view.focus();
    view.dispatch({ selection: { anchor: position } });
  }

  public override eq(other: TypstWidget) {
    return other.frame.render.hash === this.frame.render.hash;
  }

  public toDOM() {
    return this.container;
  }

  public override get estimatedHeight() {
    return this.frame.render.height;
  }
}

const compileCache = new LRUCache<string, CompileResult>({ max: 3 });

const updateFlagStore = new Set<string>();

function decorate(
  typstState: TypstState,
  update: ViewUpdate,
  path: string,
  fileId: FileId,
  prelude: string,
  widthChanged: boolean,
  locked: boolean,
  updateInWidget: boolean,
) {
  const text = update.state.doc.toString();
  const isFlagedForUpdate = updateFlagStore.has(path);

  let compileResult: CompileResult;
  if (
    update.docChanged ||
    widthChanged ||
    !compileCache.has(path) ||
    isFlagedForUpdate
  ) {
    if (update.docChanged && updateInWidget) {
      updateFlagStore.add(path);

      const diagnostics = typstState.check(fileId, text, prelude);
      dispatchDiagnostics(diagnostics, update.state, update.view);

      if (!isFlagedForUpdate) {
        updateFlagStore.add(path);
        compileResult = compileCache.get(path)!;
      } else return;
    } else {
      updateFlagStore.delete(path);

      compileResult = typstState.compile(fileId, text, prelude);
      compileCache.set(path, compileResult);
    }
  } else compileResult = compileCache.get(path)!;

  const { view, state } = update;

  const widgets: Range<Decoration>[] = [];

  dispatchDiagnostics(compileResult.diagnostics, state, view);

  for (const frame of compileResult.frames) {
    if (frame.render) {
      const { from: start, number: startLine } = state.doc.lineAt(
        frame.range.start,
      );
      const { to: end, number: endLine } = state.doc.lineAt(frame.range.end);

      const inactive =
        !view.hasFocus ||
        !updateInWidget ||
        state.selection.ranges.every(
          (range) =>
            (range.from < start || range.from > end) &&
            (range.to < start || range.to > end) &&
            (start < range.from || start > range.to) &&
            (end < range.from || end > range.to),
        );

      if (inactive) {
        const widget = new TypstWidget(view, frame, locked, fileId, typstState);
        widgets.push(Decoration.replace({ widget }).range(start, end));
      } else {
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

const typstStateEffect = StateEffect.define<{ decorations: DecorationSet }>({});

export const typstStateField = StateField.define({
  create() {
    return Decoration.none;
  },
  update(decorations, transaction) {
    const effect = transaction.effects.find((effect) =>
      effect.is(typstStateEffect),
    );
    const typstDecorations = effect?.value.decorations;

    if (typstDecorations && typstDecorations.size > 0) {
      return typstDecorations;
    }

    return decorations.map(transaction.changes);
  },
  provide: (field) => [EditorView.decorations.from(field)],
});

export const typstViewPlugin = (
  fileId: FileId,
  path: string,
  text: Ref<string>,
  prelude: Ref<string>,
  locked: boolean,
  typstState: TypstState,
) =>
  ViewPlugin.define((_view) => {
    return {
      update(update: ViewUpdate) {
        if (update.docChanged) text.value = update.state.doc.toString();

        if (
          update.docChanged ||
          update.geometryChanged ||
          update.selectionSet ||
          update.focusChanged
        ) {
          let widthChanged = false;
          if (update.geometryChanged) {
            const { scrollDOM, contentDOM } = update.view;

            widthChanged = typstState.resize(
              fileId,
              contentDOM.clientWidth - 2 * window.devicePixelRatio,
              locked ? scrollDOM.clientHeight : undefined,
            );
          }

          const state = update.state;
          const decorations = state.field(typstStateField);

          let updateInWidget = false;

          decorations.between(0, state.doc.length, (from, to) => {
            const { from: start } = state.doc.lineAt(from);
            const { to: end } = state.doc.lineAt(to);

            const active = state.selection.ranges.some(
              (range) =>
                (range.from >= start && range.from <= end) ||
                (range.to >= start && range.to <= end) ||
                (start >= range.from && start <= range.to) ||
                (end >= range.from && end <= range.to),
            );

            if (active) {
              updateInWidget = true;

              return false;
            }
          });

          queueMicrotask(() => {
            const decorations = decorate(
              typstState,
              update,
              path,
              fileId,
              prelude.value,
              widthChanged,
              locked,
              updateInWidget,
            );

            if (decorations) {
              const effects = typstStateEffect.of({ decorations });
              update.view.dispatch({ effects });
            }
          });
        }
      },
    };
  });

function dispatchDiagnostics(
  typstDiagnostics: TypstDiagnostic[],
  state: EditorState,
  view: EditorView,
) {
  if (typstDiagnostics.length > 0) {
    // const diagnostics = typstDiagnostics.map((diagnostic) => {
    //   return {
    //     from: diagnostic.range.start,
    //     to: diagnostic.range.end,
    //     severity: diagnostic.severity,
    //     message: diagnostic.message,
    //     renderMessage() {
    //       const frag = document.createDocumentFragment();
    //       const p = document.createElement("p");
    //       parseBackticks(diagnostic.message, p);
    //       frag.append(p);

    //       if (diagnostic.hints.length) {
    //         const ul = document.createElement("ul");
    //         ul.className =
    //           "text-xs pb-1 " +
    //           (diagnostic.hints.length > 1 && "list-disc list-inside");

    //         for (const hint of diagnostic.hints) {
    //           const li = document.createElement("li");
    //           parseBackticks(hint, li);
    //           ul.append(li);
    //         }

    //         frag.append(ul);
    //       }

    //       return frag;
    //     },
    //   };
    // });

    const diagnostics = typstDiagnostics.flatMap((diagnostic) => {
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
    view.dispatch(transaction);
  } else view.dispatch(setDiagnostics(state, []));
}
