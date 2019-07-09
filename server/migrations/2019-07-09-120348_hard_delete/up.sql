-- Your SQL goes here
-- notes
ALTER TABLE notes_notes DROP COLUMN deleted_at;
-- calendar
ALTER TABLE calendar_events DROP COLUMN deleted_at;
-- gallery
ALTER TABLE gallery_albums DROP COLUMN deleted_at;
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_album_id_fkey;
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_file_id_fkey;
ALTER TABLE gallery_albums_files ADD FOREIGN KEY (album_id) REFERENCES gallery_albums(id) ON DELETE CASCADE;
ALTER TABLE gallery_albums_files ADD FOREIGN KEY (file_id) REFERENCES drive_files(id) ON DELETE CASCADE;
-- music
ALTER TABLE music_playlists DROP COLUMN deleted_at;
ALTER TABLE music_playlists_files
  DROP CONSTRAINT music_playlists_files_playlist_id_fkey,
  ADD CONSTRAINT music_playlists_files_playlist_id_fkey FOREIGN KEY (playlist_id) REFERENCES music_playlists(id) ON DELETE CASCADE;
ALTER TABLE music_playlists_files
  DROP CONSTRAINT music_playlists_files_file_id_fkey,
  ADD CONSTRAINT music_playlists_files_file_id_fkey FOREIGN KEY (file_id) REFERENCES drive_files(id) ON DELETE CASCADE;
-- calendar
ALTER TABLE contacts_contacts DROP COLUMN deleted_at;
