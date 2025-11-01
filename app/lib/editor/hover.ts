import { hoverTooltip } from "@codemirror/view";
import { TypstState, FileId } from "mnemo-wasm";
// import { parseBackticks } from "./highlight";

export const typstHoverTooltip = (fileId: FileId, typstState: TypstState) =>
  hoverTooltip((_, pos, side) => {
    const tooltip = typstState.hover(fileId, pos, side);

    if (tooltip)
      return {
        pos,
        create() {
          // console.log({ tooltip });

          const div = document.createElement("div");
          div.textContent = tooltip.content;

          // if (tooltip.type == "code")
          //   div.innerHTML =
          //     "<pre>" +
          //     tooltip.content.replace(
          //       /span data-tag=(\w+)/g,
          //       (_, tag) => `span class="${"cm-highlight-" + tag}"`,
          //     ) +
          //     "</pre>";
          // else parseBackticks(tooltip.content, div);

          return { dom: div };
        },
      };
    else return null;
  });
