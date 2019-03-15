table! {
    account_accounts (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        username -> Text,
        password -> Text,
        avatar -> Text,
        is_admin -> Bool,
        recovery_id -> Nullable<Uuid>,
        recovery_token -> Nullable<Text>,
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
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        token -> Text,
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
        account_id -> Uuid,
        token -> Text,
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
        token -> Text,
        ip -> Text,
        location -> Nullable<Jsonb>,
        device -> Jsonb,
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
joinable!(drive_files -> account_accounts (owner_id));
joinable!(drive_profiles -> account_accounts (account_id));
joinable!(drive_profiles -> drive_files (home_id));
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
    drive_files,
    drive_files_events,
    drive_profiles,
    drive_profiles_events,
    phaser_reports,
    phaser_reports_events,
    phaser_scans,
    phaser_scans_events,
);
