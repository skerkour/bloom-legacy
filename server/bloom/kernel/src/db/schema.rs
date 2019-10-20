table! {
    bitflow_downloads_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        error -> Nullable<Text>,
        name -> Text,
        progress -> Int4,
        removed_at -> Nullable<Timestamptz>,
        status -> Text,
        url -> Jsonb,
        owner_id -> Uuid,
    }
}

table! {
    bitflow_profiles_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        account_id -> Uuid,
        download_folder_id -> Uuid,
    }
}

table! {
    calendar_events_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        title -> Text,
        description -> Text,
        start_at -> Timestamptz,
        end_at -> Timestamptz,
        owner_id -> Uuid,
    }
}

table! {
    contacts_contacts_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        addresses -> Array<Jsonb>,
        birthday -> Nullable<Timestamptz>,
        company -> Nullable<Text>,
        emails -> Array<Jsonb>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        notes -> Nullable<Text>,
        occupation -> Nullable<Text>,
        organizations -> Array<Jsonb>,
        phones -> Array<Jsonb>,
        websites -> Array<Jsonb>,
        owner_id -> Uuid,
    }
}

table! {
    drive_files_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        explicitly_trashed -> Bool,
        name -> Text,
        parent_id -> Nullable<Uuid>,
        size -> Int8,
        trashed_at -> Nullable<Timestamptz>,
        #[sql_name = "type"]
        type_ -> Text,
        owner_id -> Uuid,
    }
}

table! {
    drive_profiles_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        total_space -> Int8,
        used_space -> Int8,
        account_id -> Uuid,
        home_id -> Uuid,
    }
}

table! {
    drive_uploads_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        file_id -> Uuid,
        file_name -> Text,
        parent_id -> Nullable<Uuid>,
        size -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
        owner_id -> Uuid,
    }
}

table! {
    gallery_albums_files_legacy (id) {
        id -> Uuid,
        album_id -> Uuid,
        file_id -> Uuid,
    }
}

table! {
    gallery_albums_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        name -> Text,
        owner_id -> Uuid,
    }
}

table! {
    kernel_accounts (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        avatar_id -> Nullable<Text>,
        username -> Text,
        display_name -> Text,
        bio -> Text,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        is_admin -> Bool,
        disabled_at -> Nullable<Timestamptz>,
        auth_key_hash -> Text,
    }
}

table! {
    kernel_accounts_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        avatar_url -> Text,
        email -> Text,
        first_name -> Text,
        is_admin -> Bool,
        last_name -> Text,
        password -> Text,
        password_reset_id -> Nullable<Uuid>,
        password_reset_token -> Nullable<Text>,
        username -> Text,
        bio -> Text,
        display_name -> Text,
        disabled_at -> Nullable<Timestamptz>,
    }
}

table! {
    kernel_deleted_usernames (username) {
        username -> Text,
    }
}

table! {
    kernel_pending_accounts (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        email -> Text,
        display_name -> Text,
        verification_code_hash -> Text,
        trials -> Int8,
        verified -> Bool,
    }
}

table! {
    kernel_pending_accounts_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        email -> Text,
        password -> Text,
        token -> Text,
        trials -> Int8,
        verified -> Bool,
        display_name -> Text,
    }
}

table! {
    kernel_pending_emails_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        email -> Text,
        token -> Text,
        trials -> Int8,
        account_id -> Uuid,
    }
}

table! {
    kernel_sessions (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        account_id -> Uuid,
        token_hash -> Text,
        ip -> Text,
        user_agent -> Text,
    }
}

table! {
    kernel_sessions_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        device -> Jsonb,
        ip -> Text,
        location -> Nullable<Jsonb>,
        token -> Text,
        account_id -> Uuid,
    }
}

table! {
    music_playlists_files_legacy (id) {
        id -> Uuid,
        playlist_id -> Uuid,
        file_id -> Uuid,
    }
}

table! {
    music_playlists_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        name -> Text,
        owner_id -> Uuid,
    }
}

table! {
    notes_notes_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        archived_at -> Nullable<Timestamptz>,
        body -> Text,
        removed_at -> Nullable<Timestamptz>,
        title -> Text,
        owner_id -> Uuid,
    }
}

table! {
    phaser_reports_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        completed_at -> Nullable<Timestamptz>,
        error -> Nullable<Text>,
        findings -> Nullable<Array<Jsonb>>,
        high_level_findings -> Int8,
        information_findings -> Int8,
        low_level_findings -> Int8,
        medium_level_findings -> Int8,
        profile -> Text,
        started_at -> Nullable<Timestamptz>,
        status -> Text,
        targets -> Array<Text>,
        total_findings -> Int8,
        trigger -> Text,
        scan_id -> Uuid,
    }
}

table! {
    phaser_scans_legacy (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int8,
        description -> Text,
        findings -> Int8,
        last -> Nullable<Timestamptz>,
        name -> Text,
        profile -> Text,
        schedule -> Text,
        state -> Text,
        targets -> Array<Text>,
        owner_id -> Uuid,
    }
}

joinable!(bitflow_downloads_legacy -> kernel_accounts_legacy (owner_id));
joinable!(bitflow_profiles_legacy -> drive_files_legacy (download_folder_id));
joinable!(bitflow_profiles_legacy -> kernel_accounts_legacy (account_id));
joinable!(calendar_events_legacy -> kernel_accounts_legacy (owner_id));
joinable!(contacts_contacts_legacy -> kernel_accounts_legacy (owner_id));
joinable!(drive_files_legacy -> kernel_accounts_legacy (owner_id));
joinable!(drive_profiles_legacy -> drive_files_legacy (home_id));
joinable!(drive_profiles_legacy -> kernel_accounts_legacy (account_id));
joinable!(drive_uploads_legacy -> drive_files_legacy (parent_id));
joinable!(drive_uploads_legacy -> kernel_accounts_legacy (owner_id));
joinable!(gallery_albums_files_legacy -> drive_files_legacy (file_id));
joinable!(gallery_albums_files_legacy -> gallery_albums_legacy (album_id));
joinable!(gallery_albums_legacy -> kernel_accounts_legacy (owner_id));
joinable!(kernel_pending_emails_legacy -> kernel_accounts_legacy (account_id));
joinable!(kernel_sessions -> kernel_accounts (account_id));
joinable!(kernel_sessions_legacy -> kernel_accounts_legacy (account_id));
joinable!(music_playlists_files_legacy -> drive_files_legacy (file_id));
joinable!(music_playlists_files_legacy -> music_playlists_legacy (playlist_id));
joinable!(music_playlists_legacy -> kernel_accounts_legacy (owner_id));
joinable!(notes_notes_legacy -> kernel_accounts_legacy (owner_id));
joinable!(phaser_reports_legacy -> phaser_scans_legacy (scan_id));
joinable!(phaser_scans_legacy -> kernel_accounts_legacy (owner_id));

allow_tables_to_appear_in_same_query!(
    bitflow_downloads_legacy,
    bitflow_profiles_legacy,
    calendar_events_legacy,
    contacts_contacts_legacy,
    drive_files_legacy,
    drive_profiles_legacy,
    drive_uploads_legacy,
    gallery_albums_files_legacy,
    gallery_albums_legacy,
    kernel_accounts,
    kernel_accounts_legacy,
    kernel_deleted_usernames,
    kernel_pending_accounts,
    kernel_pending_accounts_legacy,
    kernel_pending_emails_legacy,
    kernel_sessions,
    kernel_sessions_legacy,
    music_playlists_files_legacy,
    music_playlists_legacy,
    notes_notes_legacy,
    phaser_reports_legacy,
    phaser_scans_legacy,
);
