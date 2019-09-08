-- This file should undo anything in `up.sql`
ALTER TABLE kernel_accounts ADD is_disabled BOOLEAN DEFAULT false;
UPDATE kernel_accounts SET is_disabled = true WHERE disabled_at IS NOT NULL OR deleted_at IS NOT NULL;
ALTER TABLE kernel_accounts DROP disabled_at;
