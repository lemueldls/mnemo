import { getMigrations } from "better-auth/db";
import { consola } from "consola";

export default defineNitroPlugin(() => {
  if (!import.meta.dev) return;

  onHubReady(async () => {
    const auth = serverAuth();
    const { toBeCreated, toBeAdded, runMigrations } = await getMigrations(
      auth.options,
    );

    if (!toBeCreated.length && !toBeAdded.length) return;

    consola.info(
      `[better-auth] Database migrations will affect the following tables:`,
    );

    for (const table of [...toBeCreated, ...toBeAdded]) {
      consola.log(
        `\`${table.table}\` table with ${Object.keys(table.fields)
          .map((f) => `\`${f}\``)
          .join(", ")} fields.`,
      );
    }

    await runMigrations();

    consola.success("[better-auth] Database migrations ran successfully");
  });
});
