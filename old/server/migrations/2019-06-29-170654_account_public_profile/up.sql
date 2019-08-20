-- Your SQL goes here
ALTER TABLE kernel_accounts ADD bio TEXT NOT NULL DEFAULT '';
ALTER TABLE kernel_accounts ADD display_name TEXT NOT NULL DEFAULT '';
UPDATE kernel_accounts SET display_name = (
  SELECT username
  FROM kernel_accounts AS ka2
  WHERE ka2.id = kernel_accounts.id
);
