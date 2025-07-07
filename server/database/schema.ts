import { integer, sqliteTable, text } from "drizzle-orm/sqlite-core";

import {
  createInsertSchema,
  createSelectSchema,
  // createUpdateSchema,
} from "drizzle-valibot";

import { cuid2, date, email, pipe, string } from "valibot";

export const users = sqliteTable("users", {
  id: text("id").notNull().primaryKey(),
  email: text("email").notNull(),
  createdAt: integer("created_at", { mode: "timestamp" }).notNull(),
});

export const insertUserSchema = createInsertSchema(users, {
  id: pipe(string(), cuid2()),
  email: pipe(string(), email()),
  createdAt: date(),
});
export const selectUserSchema = createSelectSchema(users);

// export const exports
