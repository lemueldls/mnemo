import { sqliteTable, text, integer } from "drizzle-orm/sqlite-core";

import { createInsertSchema, createSelectSchema } from "drizzle-valibot";
import { createId } from "@paralleldrive/cuid2";
import { cuid2, string, parse, number, pipe, email } from "valibot";

export const users = sqliteTable("users", {
  id: text("id", { length: 128 }).notNull().primaryKey(),
  email: text("email").notNull(),
  createdAt: integer("created_at", { mode: "timestamp" }).notNull(),
});

// Schema for inserting a user - can be used to validate API requests
export const insertUserSchema = createInsertSchema(users, {
  id: cuid2(),
  email: pipe(string(), email()),
});

// Schema for selecting a user - can be used to validate API responses
export const selectUserSchema = createSelectSchema(users);
