import { styleTags, tags as t } from "@lezer/highlight";

export const typstHighlighting = styleTags({
  String: t.string,
  Number: t.number,
  "true false": t.bool,
  PropertyName: t.propertyName,
  Null: t.null,
  ", :": t.separator,
  "[ ]": t.squareBracket,
  "{ }": t.brace,
  "$ $": t.bracket,
});
