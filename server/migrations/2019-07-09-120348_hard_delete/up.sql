-- Your SQL goes here
ALTER TABLE notes_notes DROP COLUMN deleted_at;
ALTER TABLE calendar_events DROP COLUMN deleted_at;
ALTER TABLE gallery_albums DROP COLUMN deleted_at;
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_album_id_fkey;
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_file_id_fkey;
ALTER TABLE gallery_albums_files ADD FOREIGN KEY (album_id) REFERENCES gallery_albums(id) ON DELETE CASCADE;
ALTER TABLE gallery_albums_files ADD FOREIGN KEY (file_id) REFERENCES drive_files(id) ON DELETE CASCADE;
