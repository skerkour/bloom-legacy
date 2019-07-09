-- This file should undo anything in `up.sql`
ALTER TABLE notes_notes ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE calendar_events ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
