import * as fs from "node:fs/promises";
import { fileURLToPath } from "node:url";

import symbols from "./symbols.json";

const filePath = fileURLToPath(new URL("../../app/assets/symbols.json", import.meta.url));

function titleCaseId(id: string) {
  return id
    .replace(/_/g, " ")
    .replace(/\b([a-z])/g, (m) => m.toUpperCase())
    .replace(/\b(\d[\w-]*)\b/g, (m) => m.toUpperCase());
}

const subjectSynonyms = [
  {
    re: /book|library|page|article|document/i,
    syn: ["reading", "literature", "textbook", "course"],
  },
  {
    re: /science|dna|genetics|lab|chemistry|biology|beaker|microscope/i,
    syn: ["science", "biology", "chemistry", "physics"],
  },
  {
    re: /math|calculate|function|sum|percent|analytics|graph|chart|bar_chart|area_chart/i,
    syn: ["math", "calculus", "algebra", "statistics"],
  },
  {
    re: /history|timeline|calendar|museum/i,
    syn: ["history", "historical", "humanities"],
  },
  {
    re: /language|translate|g_translate/i,
    syn: ["language", "linguistics", "english", "spanish"],
  },
  {
    re: /music|audio|mic|media|headphones/i,
    syn: ["music", "audio", "sound", "composition"],
  },
  {
    re: /code|developer|javascript|python|html|css|terminal|computer|laptop|keyboard|memory/i,
    syn: ["computer science", "programming", "coding", "chip", "cpu"],
  },
  {
    re: /art|brush|color|palette|drawing|draw/i,
    syn: ["art", "design", "visual arts"],
  },
  {
    re: /map|location|geography|globe|place/i,
    syn: ["geography", "maps", "location"],
  },
  {
    re: /finance|credit|money|currency|bank|payment/i,
    syn: ["economics", "finance", "accounting"],
  },
  {
    re: /health|medical|doctor|hospital|clinic|medication/i,
    syn: ["medicine", "health", "nursing"],
  },
  {
    re: /sports|golf|soccer|fitness|run|biking|kayaking|skiing/i,
    syn: ["sports", "physical education"],
  },
];

function generateSynonyms(id: string) {
  const syns = new Set();
  for (const m of subjectSynonyms) if (m.re.test(id)) m.syn.forEach((s) => syns.add(s));

  syns.add(id.replace(/_/g, " "));
  // syns.add(titleCaseId(id));

  return Array.from(syns).slice(0, 8);
}

const out = symbols.map((id) => ({
  id,
  title: titleCaseId(id),
  synonyms: generateSynonyms(id),
}));

await fs.writeFile(filePath, JSON.stringify(out, null, 2) + "\n", "utf8");
console.log("Wrote", out.length, "symbol objects to", filePath);
