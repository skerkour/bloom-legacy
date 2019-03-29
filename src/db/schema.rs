table! {
    account_accounts (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
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
    }
}

table! {
    account_accounts_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    account_pending_accounts (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        password -> Text,
        token -> Text,
        trials -> Int8,
        verified -> Bool,
    }
}

table! {
    account_pending_accounts_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    account_pending_emails (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        email -> Text,
        token -> Text,
        trials -> Int8,
        account_id -> Uuid,
    }
}

table! {
    account_pending_emails_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    account_sessions (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        device -> Jsonb,
        ip -> Text,
        location -> Nullable<Jsonb>,
        token -> Text,
        account_id -> Uuid,
    }
}

table! {
    account_sessions_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    bitflow_downloads (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        name -> Text,
        status -> Text,
        url -> Text,
        progress -> Int4,
        error -> Nullable<Text>,
        owner_id -> Uuid,
        file_id -> Nullable<Uuid>,
        removed_at -> Nullable<Timestamptz>,
    }
}

table! {
    bitflow_downloads_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    bitflow_profiles (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        account_id -> Uuid,
    }
}

table! {
    bitflow_profiles_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    bloom_contributors (github_login) {
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        github_login -> Text,
        commits -> Int8,
    }
}

table! {
    contacts_contacts (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        company -> Nullable<Text>,
        occupation -> Nullable<Text>,
        notes -> Nullable<Text>,
        organizations -> Nullable<Jsonb>,
        websites -> Nullable<Jsonb>,
        addresses -> Nullable<Jsonb>,
        emails -> Nullable<Jsonb>,
        phones -> Nullable<Jsonb>,
        birthday -> Nullable<Timestamptz>,
        owner_id -> Uuid,
    }
}

table! {
    contacts_contacts_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    drive_files (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        size -> Int8,
        parent_id -> Nullable<Uuid>,
        owner_id -> Uuid,
        md5 -> Nullable<Text>,
        sha1 -> Nullable<Text>,
        sha256 -> Nullable<Text>,
        sha512 -> Nullable<Text>,
        removed_at -> Nullable<Timestamptz>,
    }
}

table! {
    drive_files_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    drive_profiles (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        account_id -> Uuid,
        total_space -> Int8,
        used_space -> Int8,
        home_id -> Uuid,
    }
}

table! {
    drive_profiles_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    notes_notes (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        archived_at -> Nullable<Timestamptz>,
        body -> Text,
        removed_at -> Nullable<Timestamptz>,
        title -> Text,
        owner_id -> Uuid,
    }
}

table! {
    notes_notes_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    phaser_reports (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        scan_id -> Uuid,
        profile -> Text,
        targets -> Array<Text>,
        status -> Text,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        duration -> Int8,
        trigger -> Text,
        high_level_findings -> Int8,
        medium_level_findings -> Int8,
        low_level_findings -> Int8,
        information_findings -> Int8,
        findings -> Nullable<Jsonb>,
    }
}

table! {
    phaser_reports_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    phaser_scans (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        name -> Text,
        profile -> Text,
        targets -> Array<Text>,
        status -> Text,
        schedule -> Text,
        last -> Nullable<Timestamptz>,
        description -> Text,
        owner_id -> Uuid,
    }
}

table! {
    phaser_scans_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

joinable!(account_pending_emails -> account_accounts (account_id));
joinable!(account_sessions -> account_accounts (account_id));
joinable!(bitflow_downloads -> account_accounts (owner_id));
joinable!(bitflow_downloads -> drive_files (file_id));
joinable!(bitflow_profiles -> account_accounts (account_id));
joinable!(contacts_contacts -> account_accounts (owner_id));
joinable!(drive_files -> account_accounts (owner_id));
joinable!(drive_profiles -> account_accounts (account_id));
joinable!(drive_profiles -> drive_files (home_id));
joinable!(notes_notes -> account_accounts (owner_id));
joinable!(phaser_reports -> phaser_scans (scan_id));
joinable!(phaser_scans -> account_accounts (owner_id));

allow_tables_to_appear_in_same_query!(
    account_accounts,
    account_accounts_events,
    account_pending_accounts,
    account_pending_accounts_events,
    account_pending_emails,
    account_pending_emails_events,
    account_sessions,
    account_sessions_events,
    bitflow_downloads,
    bitflow_downloads_events,
    bitflow_profiles,
    bitflow_profiles_events,
    bloom_contributors,
    contacts_contacts,
    contacts_contacts_events,
    drive_files,
    drive_files_events,
    drive_profiles,
    drive_profiles_events,
    notes_notes,
    notes_notes_events,
    phaser_reports,
    phaser_reports_events,
    phaser_scans,
    phaser_scans_events,
);
