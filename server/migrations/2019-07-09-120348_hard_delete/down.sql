-- This file should undo anything in `up.sql`
ALTER TABLE notes_notes ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE calendar_events ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE gallery_albums ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_album_id_fkey;
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_file_id_fkey;
ALTER TABLE gallery_albums_files ADD FOREIGN KEY (album_id) REFERENCES gallery_albums(id);
ALTER TABLE gallery_albums_files ADD FOREIGN KEY (file_id) REFERENCES drive_files(id);
