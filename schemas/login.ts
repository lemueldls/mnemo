import { pipe, object, string, email, minLength, maxLength } from "valibot";

export const loginSchema = object({
  email: pipe(string(), minLength(1), maxLength(255), email()),
  password: pipe(string(), minLength(1), maxLength(255)),
});
