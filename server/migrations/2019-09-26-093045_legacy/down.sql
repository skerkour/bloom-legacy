-- This file should undo anything in `up.sql`

DROP TABLE kernel_sessions;
DROP TABLE kernel_accounts;
DROP TABLE kernel_pending_accounts;

ALTER INDEX idx_kernel_accounts_legacy_username RENAME TO idx_kernel_accounts_username;

ALTER TABLE bitflow_downloads_legacy RENAME TO bitflow_downloads;
ALTER TABLE bitflow_profiles_legacy RENAME TO bitflow_profiles;
ALTER TABLE calendar_events_legacy RENAME TO calendar_events;
ALTER TABLE contacts_contacts_legacy RENAME TO contacts_contacts;
ALTER TABLE drive_files_legacy RENAME TO drive_files;
ALTER TABLE drive_profiles_legacy RENAME TO drive_profiles;
ALTER TABLE drive_uploads_legacy RENAME TO drive_uploads;
ALTER TABLE gallery_albums_legacy RENAME TO gallery_albums;
ALTER TABLE gallery_albums_files_legacy RENAME TO gallery_albums_files;
ALTER TABLE kernel_accounts_legacy RENAME TO kernel_accounts;
ALTER TABLE kernel_pending_accounts_legacy RENAME TO kernel_pending_accounts;
ALTER TABLE kernel_pending_emails_legacy RENAME TO kernel_pending_emails;
ALTER TABLE kernel_sessions_legacy RENAME TO kernel_sessions;
ALTER TABLE music_playlists_legacy RENAME TO music_playlists;
ALTER TABLE music_playlists_files_legacy RENAME TO music_playlists_files;
ALTER TABLE notes_notes_legacy RENAME TO notes_notes;
ALTER TABLE phaser_reports_legacy RENAME TO phaser_reports;
ALTER TABLE phaser_scans_legacy RENAME TO phaser_scans;
