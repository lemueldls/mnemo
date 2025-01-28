import { sqliteTable, text, integer } from "drizzle-orm/sqlite-core";

import {
  createInsertSchema,
  createSelectSchema,
  createUpdateSchema,
} from "drizzle-valibot";
import { createId } from "@paralleldrive/cuid2";
import { cuid2, string, pipe, email } from "valibot";

export const users = sqliteTable("users", {
  id: text("id", { length: 128 }).notNull().primaryKey(),
  email: text("email").notNull(),
  createdAt: integer("created_at", { mode: "timestamp" }).notNull(),
});

export const insertUserSchema = createInsertSchema(users, {
  id: pipe(string(), cuid2()),
  email: pipe(string(), email()),
});
export const selectUserSchema = createSelectSchema(users);

// export const spaces.
