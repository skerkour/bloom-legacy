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

ALTER INDEX idx_kernel_accounts_username RENAME TO idx_kernel_accounts_legacy_username;


CREATE TABLE kernel_accounts (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    avatar_id TEXT,
    username TEXT NOT NULL,
    display_name TEXT NOT NULL,
    bio TEXT NOT NULL,

    email TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,

    is_admin BOOLEAN NOT NULL,
    disabled_at TIMESTAMP WITH TIME ZONE,
    auth_key_hash TEXT NOT NULL,
    -- password_reset_id TEXT,
    -- password_reset_token_hash TEXT,

    PRIMARY KEY(id)
);

CREATE UNIQUE INDEX idx_kernel_accounts_username ON kernel_accounts (username);

CREATE TABLE kernel_sessions (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    account_id UUID NOT NULL REFERENCES kernel_accounts(id) ON DELETE CASCADE,

    token_hash TEXT NOT NULL,
    ip TEXT NOT NULL,
    user_agent TEXT NOT NULL,

    PRIMARY KEY(id)
);

CREATE UNIQUE INDEX idx_kernel_sessions_account_id ON kernel_sessions (account_id);

CREATE TABLE kernel_pending_accounts (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    email TEXT NOT NULL,
    display_name TEXT NOT NULL,

    verification_code_hash TEXT NOT NULL,
    trials BIGINT NOT NULL,
    verified BOOLEAN NOT NULL,

    PRIMARY KEY(id)
);

