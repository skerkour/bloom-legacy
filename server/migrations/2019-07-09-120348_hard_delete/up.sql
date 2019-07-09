-- Your SQL goes here
-- notes
ALTER TABLE notes_notes DROP COLUMN deleted_at;
ALTER TABLE notes_notes
  DROP CONSTRAINT notes_notes_owner_id_fkey,
  ADD CONSTRAINT notes_notes_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;

-- calendar
ALTER TABLE calendar_events DROP COLUMN deleted_at;
ALTER TABLE calendar_events
  DROP CONSTRAINT calendar_events_owner_id_fkey,
  ADD CONSTRAINT calendar_events_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;

-- gallery
ALTER TABLE gallery_albums DROP COLUMN deleted_at;
ALTER TABLE gallery_albums
  DROP CONSTRAINT gallery_albums_owner_id_fkey,
  ADD CONSTRAINT gallery_albums_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;

ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_album_id_fkey,
  ADD CONSTRAINT gallery_albums_files_album_id_fkey FOREIGN KEY (album_id) REFERENCES gallery_albums(id) ON DELETE CASCADE;
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_file_id_fkey,
  ADD CONSTRAINT gallery_albums_files_file_id_fkey FOREIGN KEY (file_id) REFERENCES drive_files(id) ON DELETE CASCADE;

-- music
ALTER TABLE music_playlists DROP COLUMN deleted_at;
ALTER TABLE music_playlists
  DROP CONSTRAINT music_playlists_owner_id_fkey,
  ADD CONSTRAINT music_playlists_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;

ALTER TABLE music_playlists_files
  DROP CONSTRAINT music_playlists_files_playlist_id_fkey,
  ADD CONSTRAINT music_playlists_files_playlist_id_fkey FOREIGN KEY (playlist_id) REFERENCES music_playlists(id) ON DELETE CASCADE;
ALTER TABLE music_playlists_files
  DROP CONSTRAINT music_playlists_files_file_id_fkey,
  ADD CONSTRAINT music_playlists_files_file_id_fkey FOREIGN KEY (file_id) REFERENCES drive_files(id) ON DELETE CASCADE;
  -- TODO: add on cascade owner_id

-- calendar
ALTER TABLE contacts_contacts DROP COLUMN deleted_at;
ALTER TABLE contacts_contacts
  DROP CONSTRAINT contacts_contacts_owner_id_fkey,
  ADD CONSTRAINT contacts_contacts_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;


-- bitflow
ALTER TABLE bitflow_downloads DROP COLUMN deleted_at;
ALTER TABLE bitflow_downloads
  DROP CONSTRAINT bitflow_downloads_owner_id_fkey,
  ADD CONSTRAINT bitflow_downloads_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;

ALTER TABLE bitflow_profiles DROP COLUMN deleted_at;
ALTER TABLE bitflow_profiles
  DROP CONSTRAINT bitflow_profiles_account_id_fkey,
  ADD CONSTRAINT bitflow_profiles_account_id_fkey FOREIGN KEY (account_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;

-- phaser
ALTER TABLE phaser_reports DROP COLUMN deleted_at;
ALTER TABLE phaser_reports
  DROP CONSTRAINT phaser_reports_scan_id_fkey,
  ADD CONSTRAINT phaser_reports_scan_id_fkey FOREIGN KEY (scan_id) REFERENCES phaser_scans(id) ON DELETE CASCADE;

ALTER TABLE phaser_scans DROP COLUMN deleted_at;
ALTER TABLE phaser_scans
  DROP CONSTRAINT phaser_scans_owner_id_fkey,
  ADD CONSTRAINT phaser_scans_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
