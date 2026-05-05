import { StateEffect, StateField } from "@codemirror/state";
import { ViewPlugin } from "@codemirror/view";
import { showTooltip } from "@codemirror/view";

import { tooltipsStateField } from "./widgets";

import type { Tooltip } from "@codemirror/view";
import type { ViewUpdate } from "@codemirror/view";

const tooltipStateEffect = StateEffect.define<Tooltip | null>();

export const tooltipStateField = StateField.define<Tooltip | null>({
  create() {
    return null;
  },
  update(tooltip, transaction) {
    const effect = transaction.effects.find((e) => e.is(tooltipStateEffect));
    if (effect) return effect.value;
    return tooltip;
  },
  provide: (field) => showTooltip.from(field),
});

export const tooltipViewPlugin = () =>
  ViewPlugin.define((view) => ({
    update(update: ViewUpdate) {
      if (!update.selectionSet && !update.docChanged && !update.focusChanged) return;

      queueMicrotask(() => {
        if (!view.hasFocus) {
          view.dispatch({ effects: tooltipStateEffect.of(null) });

          return;
        }

        const state = view.state;
        const pos = state.selection.main.from;
        const tooltips = state.field(tooltipsStateField);

        let tooltip: Tooltip | null = null;

        // Find which frame contains the cursor
        for (const { render, range } of tooltips) {
          const { start, end } = range;

          if (pos >= start && pos <= end && render) {
            const container = document.createElement("div");
            container.classList.add("typst-popup-render");

            const svg = document.createElement("div");
            svg.style.width = render.width + "px";
            svg.style.height = render.height + "px";
            svg.style.transform = `translateX(-${render.xOffset}px)`;
            svg.setHTMLUnsafe(render.svg);

            container.append(svg);

            tooltip = {
              pos: start,
              end: end,
              create(view) {
                return {
                  dom: container,
                  getCoords(_pos) {
                    const startCoords = view.coordsAtPos(start)!;
                    const endCoords = view.coordsAtPos(end)!;

                    return {
                      left: startCoords.left,
                      right: startCoords.right,
                      top: endCoords.top,
                      bottom: endCoords.bottom,
                    };
                  },
                };
              },
            };

            break;
          }
        }

        view.dispatch({ effects: tooltipStateEffect.of(tooltip) });
      });
    },
  }));
