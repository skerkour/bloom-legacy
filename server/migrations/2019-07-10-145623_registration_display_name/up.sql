-- Your SQL goes here
DELETE FROM kernel_pending_accounts;
ALTER TABLE kernel_pending_accounts ADD COLUMN display_name TEXT NOT NULL,
  DROP COLUMN first_name, DROP COLUMN last_name;

