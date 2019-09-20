-- Your SQL goes here
-- notes
ALTER TABLE notes_notes
  DROP CONSTRAINT notes_notes_owner_id_fkey,
  ADD CONSTRAINT notes_notes_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM notes_notes WHERE deleted_at IS NOT NULL;
ALTER TABLE notes_notes DROP COLUMN deleted_at;

-- calendar
ALTER TABLE calendar_events
  DROP CONSTRAINT calendar_events_owner_id_fkey,
  ADD CONSTRAINT calendar_events_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM calendar_events WHERE deleted_at IS NOT NULL;
ALTER TABLE calendar_events DROP COLUMN deleted_at;

-- gallery
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_album_id_fkey,
  ADD CONSTRAINT gallery_albums_files_album_id_fkey FOREIGN KEY (album_id) REFERENCES gallery_albums(id) ON DELETE CASCADE;
ALTER TABLE gallery_albums_files DROP CONSTRAINT gallery_albums_files_file_id_fkey,
  ADD CONSTRAINT gallery_albums_files_file_id_fkey FOREIGN KEY (file_id) REFERENCES drive_files(id) ON DELETE CASCADE;

ALTER TABLE gallery_albums
  DROP CONSTRAINT gallery_albums_owner_id_fkey,
  ADD CONSTRAINT gallery_albums_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM gallery_albums WHERE deleted_at IS NOT NULL;
ALTER TABLE gallery_albums DROP COLUMN deleted_at;



-- music
ALTER TABLE music_playlists_files
  DROP CONSTRAINT music_playlists_files_playlist_id_fkey,
  ADD CONSTRAINT music_playlists_files_playlist_id_fkey FOREIGN KEY (playlist_id) REFERENCES music_playlists(id) ON DELETE CASCADE;
ALTER TABLE music_playlists_files
  DROP CONSTRAINT music_playlists_files_file_id_fkey,
  ADD CONSTRAINT music_playlists_files_file_id_fkey FOREIGN KEY (file_id) REFERENCES drive_files(id) ON DELETE CASCADE;


ALTER TABLE music_playlists
  DROP CONSTRAINT music_playlists_owner_id_fkey,
  ADD CONSTRAINT music_playlists_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM music_playlists WHERE deleted_at IS NOT NULL;
ALTER TABLE music_playlists DROP COLUMN deleted_at;

-- contacts
ALTER TABLE contacts_contacts
  DROP CONSTRAINT contacts_contacts_owner_id_fkey,
  ADD CONSTRAINT contacts_contacts_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM contacts_contacts WHERE deleted_at IS NOT NULL;
ALTER TABLE contacts_contacts DROP COLUMN deleted_at;

-- bitflow
ALTER TABLE bitflow_downloads
  DROP CONSTRAINT bitflow_downloads_owner_id_fkey,
  ADD CONSTRAINT bitflow_downloads_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM bitflow_downloads WHERE deleted_at IS NOT NULL;
ALTER TABLE bitflow_downloads DROP COLUMN deleted_at;

ALTER TABLE bitflow_profiles
  DROP CONSTRAINT bitflow_profiles_account_id_fkey,
  ADD CONSTRAINT bitflow_profiles_account_id_fkey FOREIGN KEY (account_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM bitflow_profiles WHERE deleted_at IS NOT NULL;
ALTER TABLE bitflow_profiles DROP COLUMN deleted_at;

-- phaser
ALTER TABLE phaser_reports
  DROP CONSTRAINT phaser_reports_scan_id_fkey,
  ADD CONSTRAINT phaser_reports_scan_id_fkey FOREIGN KEY (scan_id) REFERENCES phaser_scans(id) ON DELETE CASCADE;
DELETE FROM phaser_reports WHERE deleted_at IS NOT NULL;
ALTER TABLE phaser_reports DROP COLUMN deleted_at;

ALTER TABLE phaser_scans
  DROP CONSTRAINT phaser_scans_owner_id_fkey,
  ADD CONSTRAINT phaser_scans_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM phaser_scans WHERE deleted_at IS NOT NULL;
ALTER TABLE phaser_scans DROP COLUMN deleted_at;

-- drive
ALTER TABLE drive_uploads
  DROP CONSTRAINT drive_uploads_owner_id_fkey,
  ADD CONSTRAINT drive_uploads_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
ALTER TABLE drive_uploads
  DROP CONSTRAINT drive_uploads_parent_id_fkey,
  ADD CONSTRAINT drive_uploads_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES drive_files(id) ON DELETE CASCADE;
DELETE FROM drive_uploads WHERE deleted_at IS NOT NULL;
ALTER TABLE drive_uploads DROP COLUMN deleted_at;

ALTER TABLE drive_profiles
  DROP CONSTRAINT drive_profiles_account_id_fkey,
  ADD CONSTRAINT drive_profiles_account_id_fkey FOREIGN KEY (account_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM drive_profiles WHERE deleted_at IS NOT NULL;
ALTER TABLE drive_profiles DROP COLUMN deleted_at;


ALTER TABLE drive_files
  DROP CONSTRAINT drive_files_parent_id_fkey,
  ADD CONSTRAINT drive_files_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES drive_files(id) ON DELETE CASCADE;
ALTER TABLE drive_files
  DROP CONSTRAINT drive_files_owner_id_fkey,
  ADD CONSTRAINT drive_files_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM drive_files WHERE deleted_at IS NOT NULL;
ALTER TABLE drive_files DROP COLUMN deleted_at;

-- billing
DROP TABLE IF EXISTS billing_invoices;
DROP TABLE IF EXISTS billing_payment_methods;
DROP TABLE IF EXISTS billing_subscriptions;
DROP TABLE IF EXISTS billing_profiles;

-- myaccount
ALTER TABLE kernel_pending_emails
  DROP CONSTRAINT kernel_pending_emails_account_id_fkey,
  ADD CONSTRAINT kernel_pending_emails_account_id_fkey FOREIGN KEY (account_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM kernel_pending_emails WHERE deleted_at IS NOT NULL;
ALTER TABLE kernel_pending_emails DROP COLUMN deleted_at;

ALTER TABLE kernel_sessions
  DROP CONSTRAINT kernel_sessions_account_id_fkey,
  ADD CONSTRAINT kernel_sessions_account_id_fkey FOREIGN KEY (account_id) REFERENCES kernel_accounts(id) ON DELETE CASCADE;
DELETE FROM kernel_sessions WHERE deleted_at IS NOT NULL;
ALTER TABLE kernel_sessions DROP COLUMN deleted_at;

DELETE FROM kernel_pending_accounts WHERE deleted_at IS NOT NULL;
ALTER TABLE kernel_pending_accounts DROP COLUMN deleted_at;

DELETE FROM kernel_accounts WHERE deleted_at IS NOT NULL;
ALTER TABLE kernel_accounts DROP COLUMN deleted_at;


CREATE TABLE kernel_deleted_usernames (
    username TEXT NOT NULL,

    PRIMARY KEY(username)
);
