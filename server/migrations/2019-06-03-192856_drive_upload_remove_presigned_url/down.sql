-- This file should undo anything in `up.sql`
ALTER TABLE drive_uploads ADD presigned_url TEXT NOT NULL;
