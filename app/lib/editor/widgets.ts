import { setDiagnostics, type Diagnostic } from "@codemirror/lint";
import { StateEffect, StateField, type Range } from "@codemirror/state";
import { Decoration, EditorView, ViewPlugin, WidgetType } from "@codemirror/view";
import { LRUCache } from "lru-cache";

import { parseBackticks } from "./highlight";

import type { EditorState } from "@codemirror/state";
import type { DecorationSet, ViewUpdate } from "@codemirror/view";
import type { FileId, PagedRangedFrame, TypstDiagnostic, TypstState } from "mnemo-wasm";

const containerCache = new LRUCache<number, HTMLElement>({ max: 128 });

class TypstWidget extends WidgetType {
  container!: HTMLElement;

  public constructor(
    private readonly view: EditorView,
    private readonly frame: PagedRangedFrame,
    locked: boolean,
    private readonly fileId: FileId,
    private readonly typstState: TypstState,
  ) {
    super();

    const container = containerCache.get(frame.render.hash);

    if (container?.isConnected) {
      this.container = container;
    } else {
      const container = document.createElement("div");

      // container.attachShadow({ mode: "open" });
      // const shadow = container.shadowRoot!;

      container.dataset.hash = frame.render.hash.toString();
      container.classList.add("typst-render");
      // container.style.height = `${frame.render.height}px`;

      // const image = document.createElement("img");
      container.setHTMLUnsafe(frame.render.svg);

      // image.draggable = false;
      // image.src = `data:image/png;base64,${frame.render.encoding.toBase64()}`;
      // image.height = frame.render.height;

      if (!locked) {
        this.container.addEventListener("click", this.handleMouseEvent.bind(this));
        this.container.addEventListener("mousedown", this.handleMouseEvent.bind(this));
        // this.#container.addEventListener(
        //   "touchstart",
        //   this.handleTouchEvent.bind(this),
        // );
      }

      // container.append(image);

      containerCache.set(frame.render.hash, container);
      this.container = container;
    }
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
    const { top, left } = this.container.getBoundingClientRect();

    const x = clientX - left;
    const y = clientY - top;

    const jump = typstState.jumpPaged(this.fileId, x, y + frame.render.offsetHeight);
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
}

const framesCache = new LRUCache<string, PagedRangedFrame[]>({ max: 8 });

const updateFlagStore = new Set<string>();

function decorate(
  fileId: FileId,
  spaceId: string,
  path: string,
  prelude: string,
  locked: boolean,
  update: ViewUpdate,
  updateInWidget: boolean,
  widthChanged: boolean,
  typstState: TypstState,
) {
  const text = update.state.doc.toString();
  const isFlaggedForUpdate = updateFlagStore.has(path);

  let frames: PagedRangedFrame[];

  if (update.docChanged || widthChanged || !framesCache.has(path) || isFlaggedForUpdate) {
    if (update.docChanged && updateInWidget && isFlaggedForUpdate) {
      const { diagnostics, requests } = typstState.checkPaged(fileId, text, prelude);
      dispatchDiagnostics(diagnostics, update.state, update.view);

    if (requests.length > 0)
      handleTypstRequests(requests, spaceId).then((update) => {
        if (update) {
          view.dispatch({ changes: [{ from: 0, insert: "\n" }] });
          view.dispatch({ changes: [{ from: 0, to: 1 }] });
        }
      });

      return;
    } else {
    if (isFlaggedForUpdate) updateFlagStore.delete(path);
    else updateFlagStore.add(path);

    const compileResult = typstState.compilePaged(fileId, text, prelude);
    dispatchDiagnostics(compileResult.diagnostics, update.state, update.view);

    console.log(compileResult);

    if (compileResult.requests.length > 0)
      handleTypstRequests(compileResult.requests, spaceId).then((update) => {
        if (update) {
          view.dispatch({ changes: [{ from: 0, insert: "\n" }] });
          view.dispatch({ changes: [{ from: 0, to: 1 }] });
        }
      });

    frames = compileResult.frames;
    framesCache.set(path, compileResult.frames);
    }
  } else frames = framesCache.get(path)!;

  const { view, state } = update;

  const decorations: Range<Decoration>[] = [];

  for (const frame of frames) {
    const { start, end } = frame.range;

    const { from, number: startLine } = state.doc.lineAt(start);
    const { to, number: endLine } = state.doc.lineAt(end);

    if (frame.render) {
      const inactive =
        !view.hasFocus ||
        state.selection.ranges.every(
          (range) =>
            (range.from < start || range.from > end) &&
            (range.to < start || range.to > end) &&
            (start < range.from || start > range.to) &&
            (end < range.from || end > range.to),
        );

      if (inactive) {
        const widget = new TypstWidget(view, frame, locked, fileId, typstState);

        decorations.push(Decoration.replace({ widget }).range(start, end));
      } else {
        let lineHeight = 0;

        const { number: startLine } = state.doc.lineAt(start);
        const { number: endLine } = state.doc.lineAt(end);

        for (let currentLine = startLine; currentLine <= endLine; currentLine++) {
          const line = state.doc.line(currentLine);
          let style = "";
          if (currentLine == startLine)
            style += "border-top-left-radius:0.25rem;border-top-right-radius:0.25rem;";
          if (currentLine == endLine)
            style += `border-bottom-left-radius:0.25rem;border-bottom-right-radius:0.25rem;min-height:${
              height ? height - lineHeight : lineHeight
            }px`;
          else {
            lineHeight += view.lineBlockAt(line.from).height;
          }

          decorations.push(
            Decoration.line({
              class: "cm-activeLine",
              attributes: { style },
            }).range(line.from),
          );
        }
      }
    }
  }

  return Decoration.set(decorations, true);
}

const typstStateEffect = StateEffect.define<{ decorations: DecorationSet }>({});

export const typstStateField = StateField.define({
  create() {
    return Decoration.none;
  },
  update(decorations, transaction) {
    const effect = transaction.effects.find((effect) => effect.is(typstStateEffect));

    if (effect) return effect.value.decorations;
    return decorations.map(transaction.changes);
  },
  provide: (field) => [EditorView.decorations.from(field)],
});

export const typstViewPlugin = (
  fileId: FileId,
  spaceId: string,
  path: string,
  text: Ref<string>,
  prelude: Ref<string>,
  locked: boolean,
  typstState: TypstState,
) =>
  ViewPlugin.define((_view) => {
    return {
      update(update: ViewUpdate) {
        let widthChanged = false;
        if (update.geometryChanged) {
          const { scrollDOM, contentDOM } = update.view;

          widthChanged = typstState.resize(
            fileId,
            contentDOM.clientWidth - 2 * window.devicePixelRatio,
            locked ? scrollDOM.clientHeight : undefined,
          );
        }

        if (update.docChanged || update.selectionSet || update.focusChanged || widthChanged) {
          const state = update.state;
          const currentDecorations = state.field(typstStateField);

          let updateInWidget = false;

          currentDecorations.between(0, state.doc.length, (from, to) => {
            if (to > state.doc.length) return;

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
              fileId,
              spaceId,
              path,
              prelude.value,
              locked,
              update,
              updateInWidget,
              widthChanged,
              typstState,
            );

            if (decorations) {
              const effects = typstStateEffect.of({ decorations });
              update.view.dispatch({ effects });
            }
          });

          if (update.docChanged) text.value = update.state.doc.toString();
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
    const diagnostics = typstDiagnostics.map((diagnostic) => {
      return {
        from: diagnostic.range.start,
        to: diagnostic.range.end,
        severity: diagnostic.severity,
        message: diagnostic.message,
        renderMessage() {
          const frag = document.createDocumentFragment();
          const p = document.createElement("p");
          parseBackticks(diagnostic.message, p);
          frag.append(p);

          if (diagnostic.hints.length) {
            const ul = document.createElement("ul");
            ul.className = "typst-hints";

            for (const hint of diagnostic.hints) {
              const li = document.createElement("li");
              parseBackticks(hint, li);
              ul.append(li);
            }

            frag.append(ul);
          }

          return frag;
        },
      };
    });

    const transaction = setDiagnostics(state, diagnostics);
    view.dispatch(transaction);
  } else view.dispatch(setDiagnostics(state, []));
}
