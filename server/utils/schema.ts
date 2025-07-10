import {
  safeParseAsync,
  type GenericIssue,
  type GenericSchema,
  type GenericSchemaAsync,
} from "valibot";

import type { H3Event } from "h3";

export async function validatedBody<
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

export async function validatedQuery<
  const TSchema extends
    | GenericSchema<unknown, unknown, GenericIssue>
    | GenericSchemaAsync<unknown, unknown, GenericIssue>,
>(event: H3Event, schema: TSchema) {
  const { success, output, issues } = await getValidatedQuery(event, (query) =>
    safeParseAsync(schema, query),
  );

  if (success) return output;

  throw createError({
    status: 422,
    data: issues,
    fatal: false,
    unhandled: false,
  });
}

export async function validatedRouterParams<
  const TSchema extends
    | GenericSchema<unknown, unknown, GenericIssue>
    | GenericSchemaAsync<unknown, unknown, GenericIssue>,
>(event: H3Event, schema: TSchema, options?: { decode?: boolean }) {
  const { success, output, issues } = await getValidatedRouterParams(
    event,
    (query) => safeParseAsync(schema, query),
    options,
  );

  if (success) return output;

  throw createError({
    status: 422,
    data: issues,
    fatal: false,
    unhandled: false,
  });
}

export async function validatedFormData<
  const TSchema extends
    | GenericSchema<unknown, unknown, GenericIssue>
    | GenericSchemaAsync<unknown, unknown, GenericIssue>,
>(event: H3Event, schema: TSchema) {
  const formData = await readFormData(event);

  const { success, output, issues } = await safeParseAsync(
    schema,
    Object.fromEntries(formData),
  );

  if (success) return output;

  throw createError({
    status: 422,
    data: issues,
    fatal: false,
    unhandled: false,
  });
}
