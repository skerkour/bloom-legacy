-- Your SQL goes here

ALTER TABLE bitflow_downloads RENAME TO bitflow_downloads_legacy;
ALTER TABLE bitflow_profiles RENAME TO bitflow_profiles_legacy;
ALTER TABLE calendar_events RENAME TO calendar_events_legacy;
ALTER TABLE contacts_contacts RENAME TO contacts_contacts_legacy;
ALTER TABLE drive_files RENAME TO drive_files_legacy;
ALTER TABLE drive_profiles RENAME TO drive_profiles_legacy;
ALTER TABLE drive_uploads RENAME TO drive_uploads_legacy;
ALTER TABLE gallery_albums RENAME TO gallery_albums_legacy;
ALTER TABLE gallery_albums_files RENAME TO gallery_albums_files_legacy;
ALTER TABLE kernel_accounts RENAME TO kernel_accounts_legacy;
ALTER TABLE kernel_pending_accounts RENAME TO kernel_pending_accounts_legacy;
ALTER TABLE kernel_pending_emails RENAME TO kernel_pending_emails_legacy;
ALTER TABLE kernel_sessions RENAME TO kernel_sessions_legacy;
ALTER TABLE music_playlists RENAME TO music_playlists_legacy;
ALTER TABLE music_playlists_files RENAME TO music_playlists_files_legacy;
ALTER TABLE notes_notes RENAME TO notes_notes_legacy;
ALTER TABLE phaser_reports RENAME TO phaser_reports_legacy;
ALTER TABLE phaser_scans RENAME TO phaser_scans_legacy;
