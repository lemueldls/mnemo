import { hoverTooltip } from "@codemirror/view";

import { parseBackticks } from "./highlight";

import type { FileId, TypstState } from "mnemo-wasm";

export const typstHoverTooltip = (fileId: FileId, typstState: TypstState) =>
  hoverTooltip((_, pos, side) => {
    const tooltip = typstState.hover(fileId, pos, side);

    if (tooltip)
      return {
        pos,
        create() {
          const div = document.createElement("div");

          if (tooltip.startsWith("<code>"))
            div.innerHTML =
              "<pre>" +
              tooltip.replace(/span data-tag=(\w+)/g, (_, tag) => `span class="${"typ-" + tag}"`) +
              "</pre>";
          else parseBackticks(tooltip, div);

          return { dom: div };
        },
      };
    else return null;
  });
