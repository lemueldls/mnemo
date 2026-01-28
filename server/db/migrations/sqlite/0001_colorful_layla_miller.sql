ALTER TABLE `account` RENAME TO `accounts`;--> statement-breakpoint
ALTER TABLE `user` RENAME TO `users`;--> statement-breakpoint
ALTER TABLE `verification` RENAME TO `verifications`;--> statement-breakpoint
ALTER TABLE `accounts` RENAME COLUMN "accountId" TO "account_id";--> statement-breakpoint
ALTER TABLE `accounts` RENAME COLUMN "providerId" TO "provider_id";--> statement-breakpoint
ALTER TABLE `accounts` RENAME COLUMN "userId" TO "user_id";--> statement-breakpoint
ALTER TABLE `accounts` RENAME COLUMN "accessToken" TO "access_token";--> statement-breakpoint
ALTER TABLE `accounts` RENAME COLUMN "refreshToken" TO "refresh_token";--> statement-breakpoint
ALTER TABLE `accounts` RENAME COLUMN "idToken" TO "id_token";--> statement-breakpoint
ALTER TABLE `accounts` RENAME COLUMN "accessTokenExpiresAt" TO "access_token_expires_at";--> statement-breakpoint
ALTER TABLE `accounts` RENAME COLUMN "refreshTokenExpiresAt" TO "refresh_token_expires_at";--> statement-breakpoint
ALTER TABLE `accounts` RENAME COLUMN "createdAt" TO "created_at";--> statement-breakpoint
ALTER TABLE `accounts` RENAME COLUMN "updatedAt" TO "updated_at";--> statement-breakpoint
ALTER TABLE `users` RENAME COLUMN "emailVerified" TO "email_verified";--> statement-breakpoint
ALTER TABLE `users` RENAME COLUMN "createdAt" TO "created_at";--> statement-breakpoint
ALTER TABLE `users` RENAME COLUMN "updatedAt" TO "updated_at";--> statement-breakpoint
ALTER TABLE `verifications` RENAME COLUMN "expiresAt" TO "expires_at";--> statement-breakpoint
ALTER TABLE `verifications` RENAME COLUMN "createdAt" TO "created_at";--> statement-breakpoint
ALTER TABLE `verifications` RENAME COLUMN "updatedAt" TO "updated_at";--> statement-breakpoint
PRAGMA foreign_keys=OFF;--> statement-breakpoint
CREATE TABLE `__new_accounts` (
	`id` text PRIMARY KEY NOT NULL,
	`account_id` text NOT NULL,
	`provider_id` text NOT NULL,
	`user_id` text NOT NULL,
	`access_token` text,
	`refresh_token` text,
	`id_token` text,
	`access_token_expires_at` integer,
	`refresh_token_expires_at` integer,
	`scope` text,
	`password` text,
	`created_at` integer DEFAULT (cast(unixepoch('subsecond') * 1000 as integer)) NOT NULL,
	`updated_at` integer NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON UPDATE no action ON DELETE cascade
);
--> statement-breakpoint
INSERT INTO `__new_accounts`("id", "account_id", "provider_id", "user_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at") SELECT "id", "account_id", "provider_id", "user_id", "access_token", "refresh_token", "id_token", "access_token_expires_at", "refresh_token_expires_at", "scope", "password", "created_at", "updated_at" FROM `accounts`;--> statement-breakpoint
DROP TABLE `accounts`;--> statement-breakpoint
ALTER TABLE `__new_accounts` RENAME TO `accounts`;--> statement-breakpoint
PRAGMA foreign_keys=ON;--> statement-breakpoint
CREATE INDEX `accounts_userId_idx` ON `accounts` (`user_id`);--> statement-breakpoint
DROP INDEX `user_email_unique`;--> statement-breakpoint
CREATE UNIQUE INDEX `users_email_unique` ON `users` (`email`);--> statement-breakpoint
DROP INDEX `verification_identifier_idx`;--> statement-breakpoint
CREATE INDEX `verifications_identifier_idx` ON `verifications` (`identifier`);