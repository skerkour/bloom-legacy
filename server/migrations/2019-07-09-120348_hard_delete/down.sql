-- This file should undo anything in `up.sql`
-- notes
ALTER TABLE notes_notes ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
-- calendar
ALTER TABLE calendar_events ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
-- album
ALTER TABLE gallery_albums ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE gallery_albums_files
  DROP CONSTRAINT gallery_albums_files_album_id_fkey,
  ADD CONSTRAINT gallery_albums_files_album_id_fkey FOREIGN KEY (album_id) REFERENCES gallery_albums(id);
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_file_id_fkey,
  ADD CONSTRAINT gallery_albums_files_file_id_fkey FOREIGN KEY (file_id) REFERENCES drive_files(id);
-- music
ALTER TABLE music_playlists ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE music_playlists_files DROP CONSTRAINT music_playlists_files_playlist_id_fkey,
  ADD CONSTRAINT music_playlists_files_playlist_id_fkey FOREIGN KEY (playlist_id) REFERENCES music_playlists(id);
ALTER TABLE music_playlists_files DROP CONSTRAINT music_playlists_files_file_id_fkey,
  ADD CONSTRAINT music_playlists_files_file_id_fkey FOREIGN KEY (file_id) REFERENCES drive_files(id);
-- contacts
ALTER TABLE contacts_contacts ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
-- contacts
ALTER TABLE bitflow_downloads ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE bitflow_downloads DROP CONSTRAINT bitflow_downloads_owner_id_fkey,
  ADD CONSTRAINT bitflow_downloads_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id);
ALTER TABLE bitflow_profiles ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE bitflow_profiles DROP CONSTRAINT bitflow_profiles_account_id_fkey,
  ADD CONSTRAINT bitflow_profiles_account_id_fkey FOREIGN KEY (account_id) REFERENCES kernel_accounts(id);
