import * as fs from "node:fs/promises";
import { fileURLToPath } from "node:url";

import metadata from "./metadata.json";

const MS_FAMILIES = [
  "Material Symbols Outlined",
  "Material Symbols Rounded",
  "Material Symbols Sharp",
] as const;

function toTitle(name: string) {
  return name
    .split("_")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

const filePath = fileURLToPath(new URL("../../app/assets/symbols.json", import.meta.url));

const icons = (
  metadata.icons as {
    name: string;
    tags: string[];
    unsupported_families: string[];
  }[]
)
  .filter((icon) => {
    const unsupported = new Set(icon.unsupported_families);
    return !MS_FAMILIES.every((f) => unsupported.has(f));
  })
  .map((icon) => ({
    id: icon.name,
    title: toTitle(icon.name),
    synonyms: icon.tags,
  }));

await fs.writeFile(filePath, JSON.stringify(icons));
console.log(`Wrote ${icons.length} symbols to ${filePath}`);
