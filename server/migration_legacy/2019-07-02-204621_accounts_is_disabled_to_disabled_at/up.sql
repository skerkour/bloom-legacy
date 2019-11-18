-- Your SQL goes here
ALTER TABLE kernel_accounts ADD disabled_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
UPDATE kernel_accounts SET disabled_at = now() WHERE is_disabled = true AND deleted_at IS NULL;
UPDATE kernel_accounts SET disabled_at = deleted_at WHERE deleted_at IS NOT NULL;
ALTER TABLE kernel_accounts DROP is_disabled;
