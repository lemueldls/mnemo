import { MaterialDynamicColors } from "@material/material-color-utilities";

import {
  breakpointsM3,
  verticalBreakpointsM3,
} from "./composables/breakpoints";

import type { Preset } from "unocss";

function kebabCase(text: string) {
  return text.replaceAll(
    /[a-z][A-Z]/g,
    (match) => `${match[0]}-${match[1].toLowerCase()}`,
  );
}

const colors = {} as { [key: string]: string };

for (const color in MaterialDynamicColors)
  if (Object.hasOwn(MaterialDynamicColors, color)) {
    const key = kebabCase(color);

    colors[`m3-${key}`] = `var(--md-sys-color-${key})`;
  }

export function presetM3(): Preset {
  return {
    name: "m3",
    prefix: "m3-",
    rules: [
      [
        "display-large",
        {
          "line-height": "4rem",
          "font-size": "3.5625rem",
          "letter-spacing": "-0.015625rem",
          "font-weight": "400",
        },
      ],
      [
        "display-medium",
        {
          "line-height": "3.25rem",
          "font-size": "2.8125rem",
          "letter-spacing": "0",
          "font-weight": "400",
        },
      ],
      [
        "display-small",
        {
          "line-height": "2.5rem",
          "font-size": "2.25rem",
          "letter-spacing": "0",
          "font-weight": "400",
        },
      ],
      [
        "headline-large",
        {
          "line-height": "2.5rem",
          "font-size": "2rem",
          "letter-spacing": "0",
          "font-weight": "400",
        },
      ],
      [
        "headline-medium",
        {
          "line-height": "2.25rem",
          "font-size": "1.75rem",
          "letter-spacing": "0",
          "font-weight": "400",
        },
      ],
      [
        "headline-small",
        {
          "line-height": "2rem",
          "font-size": "1.5rem",
          "letter-spacing": "0",
          "font-weight": "400",
        },
      ],
      [
        "title-large",
        {
          "line-height": "1.75rem",
          "font-size": "1.375rem",
          "letter-spacing": "0",
          "font-weight": "400",
        },
      ],
      [
        "title-medium",
        {
          "line-height": "1.5rem",
          "font-size": "1rem",
          "letter-spacing": "0.009375rem",
          "font-weight": "500",
        },
      ],
      [
        "title-small",
        {
          "line-height": "1.25rem",
          "font-size": "0.875rem",
          "letter-spacing": "0.00625rem",
          "font-weight": "500",
        },
      ],
      [
        "label-large",
        {
          "line-height": "1.25rem",
          "font-size": "0.875rem",
          "letter-spacing": "0.00625rem",
          "font-weight": "500",
        },
      ],
      [
        "label-medium",
        {
          "line-height": "1rem",
          "font-size": "0.75rem",
          "letter-spacing": "0.03125rem",
          "font-weight": "500",
        },
      ],
      [
        "label-small",
        {
          "line-height": "1rem",
          "font-size": "0.6875rem",
          "letter-spacing": "0.03125rem",
          "font-weight": "500",
        },
      ],
      [
        "body-large",
        {
          "line-height": "1.25rem",
          "font-size": "1rem",
          "letter-spacing": "0.03125rem",
          "font-weight": "400",
        },
      ],
      [
        "body-medium",
        {
          "line-height": "1.25rem",
          "font-size": "0.875rem",
          "letter-spacing": "0.015625rem",
          "font-weight": "400",
        },
      ],
      [
        "body-small",
        {
          "line-height": "1rem",
          "font-size": "0.75rem",
          "letter-spacing": "0.025rem",
          "font-weight": "400",
        },
      ],
    ],
    theme: {
      colors,
      breakpoints: breakpointsM3,
      verticalBreakpoints: verticalBreakpointsM3,
    },
  };
}
