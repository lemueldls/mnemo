import { syntaxTree } from "@codemirror/language";

import {
  Decoration,
  ViewPlugin,
  WidgetType,
  EditorView,
} from "@codemirror/view";

import { StateEffect, StateField } from "@codemirror/state";

import type { TypstState, FileId, Block } from "mnemo-wasm";

import type { ViewUpdate, DecorationSet } from "@codemirror/view";

import type { Range } from "@codemirror/state";
import { setDiagnostics, type Diagnostic } from "@codemirror/lint";

class TypstWidget extends WidgetType {
  #image = document.createElement("img");

  public constructor(
    private readonly typstState: TypstState,
    private readonly view: EditorView,
    private readonly index: number,
    private readonly render: string,
    private readonly block: Block,
  ) {
    super();

    // this.#image.style.width = `${view.dom.clientWidth}px`;
    // this.#image.style.display = "inline";
    // this.#image.style.verticalAlign = "bottom";
    // this.#image.style.cursor = "text";
    this.#image.draggable = false;

    this.#image.classList.add("typst-render");

    this.#image.addEventListener("click", this.handleJump.bind(this));
    // this.#image.addEventListener("mousedown", this.handleJump.bind(this));
  }

  private async handleJump(event: MouseEvent) {
    event.preventDefault();

    const { typstState, index, block, view } = this;
    const { top, left } = this.#image.getBoundingClientRect();

    const x = event.clientX - left;
    const y = event.clientY - top;

    const jump = typstState.click(index, x, y); // can crash
    const position = jump
      ? block.range.start + (jump.position - block.offset)
      : block.range.end;

    view.dispatch({ selection: { anchor: position } });
  }

  public override eq(other: TypstWidget) {
    return other.render === this.render;
  }

  public toDOM() {
    this.#image.src = `data:image/png;base64,${this.render}`;

    return this.#image;
  }

  public override get estimatedHeight() {
    return this.#image.height;
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

  const diagnostics = syncResult.flatMap(({ block }) =>
    block.errors.flatMap((diagnostic) => {
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
    }),
  );
  const transaction = setDiagnostics(state, diagnostics);
  queueMicrotask(() => view.dispatch(transaction));

  console.log({ syncResult, diagnostics });

  for (const { index, block, render } of syncResult) {
    // console.log({ index, block, render });

    if (render) {
      const { start, end } = block.range;
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
            widget: new TypstWidget(typstState, view, index, render, block),
            // inclusive: true,
          }).range(start, end),
        );
      else {
        for (let i = start; i < end; i++) {
          const line = state.doc.lineAt(i);
          const from = line.from;
          // const to = line.to;

          let style = "";
          if (i == start)
            style +=
              "border-top-left-radius:0.25rem;border-top-right-radius:0.25rem";
          if (i == end - 1)
            style +=
              "border-bottom-left-radius:0.25rem;border-bottom-right-radius:0.25rem";

          widgets.push(
            Decoration.line({
              class: "cm-activeLine",
              attributes: { style },
              // inclusive: true,
            }).range(from),
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
          typstState.resize(update.view.dom.clientWidth - 1);

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

    window.location.reload();

    throw error;
  }

  return result;
}
