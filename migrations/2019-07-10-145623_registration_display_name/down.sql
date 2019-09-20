-- This file should undo anything in `up.sql`
ALTER TABLE kernel_pending_accounts ADD COLUMN first_name TEXT NOT NULL,
  ADD COLUMN last_name TEXT NOT NULL, DROP COLUMN display_name;

