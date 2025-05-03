import type { H3Event } from "h3";
import {
  safeParseAsync,
  type GenericSchema,
  type GenericSchemaAsync,
  type GenericIssue,
} from "valibot";

export async function readBodyWithSchema<
  const TSchema extends
    | GenericSchema<unknown, unknown, GenericIssue>
    | GenericSchemaAsync<unknown, unknown, GenericIssue>,
>(event: H3Event, schema: TSchema) {
  const { success, output, issues } = await readValidatedBody(event, (body) =>
    safeParseAsync(schema, body),
  );

  if (success) return output;

  throw createError({
    status: 422,
    data: issues,
    fatal: false,
    unhandled: false,
  });
}
