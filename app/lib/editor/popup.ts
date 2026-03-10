import { StateEffect, StateField } from "@codemirror/state";
import { ViewPlugin } from "@codemirror/view";
import { showTooltip } from "@codemirror/view";

import { tooltipsStateField } from "./widgets";

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
            svg.style.transform = `translate(-${render.xOffset}px)`;
            svg.setHTMLUnsafe(render.svg);

            container.append(svg);

            tooltip = {
              pos: start,
              end: end,
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
