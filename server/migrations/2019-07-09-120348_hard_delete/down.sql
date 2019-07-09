-- This file should undo anything in `up.sql`
-- notes
ALTER TABLE notes_notes ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE notes_notes DROP CONSTRAINT notes_notes_owner_id_fkey,
  ADD CONSTRAINT notes_notes_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id);

-- calendar
ALTER TABLE calendar_events ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE calendar_events DROP CONSTRAINT calendar_events_owner_id_fkey,
  ADD CONSTRAINT calendar_events_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id);

-- album
ALTER TABLE gallery_albums ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE gallery_albums DROP CONSTRAINT gallery_albums_owner_id_fkey,
  ADD CONSTRAINT gallery_albums_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id);

ALTER TABLE gallery_albums_files
  DROP CONSTRAINT gallery_albums_files_album_id_fkey,
  ADD CONSTRAINT gallery_albums_files_album_id_fkey FOREIGN KEY (album_id) REFERENCES gallery_albums(id);
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_file_id_fkey,
  ADD CONSTRAINT gallery_albums_files_file_id_fkey FOREIGN KEY (file_id) REFERENCES drive_files(id);

-- music
ALTER TABLE music_playlists ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE music_playlists DROP CONSTRAINT music_playlists_owner_id_fkey,
  ADD CONSTRAINT music_playlists_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id);

ALTER TABLE music_playlists_files DROP CONSTRAINT music_playlists_files_playlist_id_fkey,
  ADD CONSTRAINT music_playlists_files_playlist_id_fkey FOREIGN KEY (playlist_id) REFERENCES music_playlists(id);
ALTER TABLE music_playlists_files DROP CONSTRAINT music_playlists_files_file_id_fkey,
  ADD CONSTRAINT music_playlists_files_file_id_fkey FOREIGN KEY (file_id) REFERENCES drive_files(id);

-- contacts
ALTER TABLE contacts_contacts ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE contacts_contacts DROP CONSTRAINT contacts_contacts_owner_id_fkey,
  ADD CONSTRAINT contacts_contacts_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id);

-- contacts
ALTER TABLE bitflow_downloads ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE bitflow_downloads DROP CONSTRAINT bitflow_downloads_owner_id_fkey,
  ADD CONSTRAINT bitflow_downloads_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id);

ALTER TABLE bitflow_profiles ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE bitflow_profiles DROP CONSTRAINT bitflow_profiles_account_id_fkey,
  ADD CONSTRAINT bitflow_profiles_account_id_fkey FOREIGN KEY (account_id) REFERENCES kernel_accounts(id);

-- phaser
ALTER TABLE phaser_reports ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE phaser_reports DROP CONSTRAINT phaser_reports_scan_id_fkey,
  ADD CONSTRAINT phaser_reports_scan_id_fkey FOREIGN KEY (scan_id) REFERENCES phaser_scans(id);

ALTER TABLE phaser_scans ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE phaser_scans DROP CONSTRAINT phaser_scans_owner_id_fkey,
  ADD CONSTRAINT phaser_scans_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id);

-- drive
ALTER TABLE drive_uploads ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE drive_uploads DROP CONSTRAINT drive_uploads_owner_id_fkey,
  ADD CONSTRAINT drive_uploads_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id);
ALTER TABLE drive_uploads DROP CONSTRAINT drive_uploads_parent_id_fkey,
  ADD CONSTRAINT drive_uploads_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES drive_files(id);

ALTER TABLE drive_profiles ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE drive_profiles DROP CONSTRAINT drive_profiles_account_id_fkey,
  ADD CONSTRAINT drive_profiles_account_id_fkey FOREIGN KEY (account_id) REFERENCES kernel_accounts(id);

ALTER TABLE drive_files ADD deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
ALTER TABLE drive_files DROP CONSTRAINT drive_files_owner_id_fkey,
  ADD CONSTRAINT drive_files_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id);
