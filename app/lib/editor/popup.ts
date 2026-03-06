import { StateEffect, StateField } from "@codemirror/state";
import { ViewPlugin } from "@codemirror/view";
import { showTooltip } from "@codemirror/view";

import { framesStateField } from "./widgets";

import type { Tooltip } from "@codemirror/view";
import type { ViewUpdate } from "@codemirror/view";

const popupTooltipEffect = StateEffect.define<Tooltip | null>();

export const popupStateField = StateField.define<Tooltip | null>({
  create() {
    return null;
  },
  update(tooltip, transaction) {
    const effect = transaction.effects.find((e) => e.is(popupTooltipEffect));
    if (effect) return effect.value;
    return tooltip;
  },
  provide: (field) => showTooltip.from(field),
});

export const popupViewPlugin = () =>
  ViewPlugin.define((view) => ({
    update(update: ViewUpdate) {
      if (!update.selectionSet && !update.docChanged) return;

      queueMicrotask(() => {
        const state = view.state;
        const pos = state.selection.main.from;
        const frames = state.field(framesStateField);

        let tooltip: Tooltip | null = null;

        // Find which frame contains the cursor
        for (const frame of frames) {
          const { start, end } = frame.range;

          if (pos >= start && pos <= end && frame.render) {
            const container = document.createElement("div");
            container.style.width = frame.render.width + "px";
            container.style.height = frame.render.height + "px";
            container.style.overflow = "visible";

            const { contentDOM } = update.view;

            const svg = document.createElement("div");
            svg.classList.add("typst-popup-render");
            svg.style.width = contentDOM.clientWidth - 2 + "px";
            // svg.style.height = frame.render.height + "px";
            svg.style.transform = `translate(-${frame.render.xOffset}px)`;
            svg.setHTMLUnsafe(frame.render.svg);

            container.append(svg);

            const { doc } = state;
            const { from: lineFrom, to: lineTo } = doc.lineAt(start);

            tooltip = {
              pos: lineFrom,
              end: lineTo,
              // above: false,
              // arrow: true,
              // strictSide: true,
              create() {
                return { dom: container };
              },
            };

            break;
          }
        }

        view.dispatch({
          effects: popupTooltipEffect.of(tooltip),
        });
      });
    },
  }));
