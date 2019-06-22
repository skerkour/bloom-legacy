table! {
    billing_invoices (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        details -> Jsonb,
        billing_profile_id -> Uuid,
    }
}

table! {
    billing_invoices_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    billing_payment_methods (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        details -> Jsonb,
        is_default -> Bool,
        billing_profile_id -> Uuid,
    }
}

table! {
    billing_payment_methods_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    billing_profiles (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        stripe_customer_id -> Nullable<Text>,
        account_id -> Uuid,
    }
}

table! {
    billing_profiles_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    billing_subscriptions (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        billing_profile_id -> Uuid,
    }
}

table! {
    billing_subscriptions_events (id) {
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
        download_folder_id -> Uuid,
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
    calendar_events (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        title -> Text,
        description -> Text,
        start_at -> Timestamptz,
        end_at -> Timestamptz,
        owner_id -> Uuid,
    }
}

table! {
    calendar_events_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    contacts_contacts (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
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
        total_space -> Int8,
        used_space -> Int8,
        account_id -> Uuid,
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
    drive_uploads (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
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
    drive_uploads_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    gallery_albums (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        name -> Text,
        owner_id -> Uuid,
    }
}

table! {
    gallery_albums_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    gallery_albums_files (id) {
        id -> Uuid,
        album_id -> Uuid,
        file_id -> Uuid,
    }
}

table! {
    kernel_accounts (id) {
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
        is_disabled -> Bool,
    }
}

table! {
    kernel_accounts_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    kernel_pending_accounts (id) {
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
    kernel_pending_accounts_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    kernel_pending_emails (id) {
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
    kernel_pending_emails_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    kernel_sessions (id) {
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
    kernel_sessions_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    music_playlists (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        version -> Int8,
        name -> Text,
        owner_id -> Uuid,
    }
}

table! {
    music_playlists_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

table! {
    music_playlists_files (id) {
        id -> Uuid,
        playlist_id -> Uuid,
        file_id -> Uuid,
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

table! {
    phaser_scans_events (id) {
        id -> Uuid,
        timestamp -> Timestamptz,
        aggregate_id -> Uuid,
        data -> Jsonb,
        metadata -> Jsonb,
    }
}

joinable!(billing_invoices -> billing_profiles (billing_profile_id));
joinable!(billing_invoices_events -> billing_invoices (aggregate_id));
joinable!(billing_payment_methods -> billing_profiles (billing_profile_id));
joinable!(billing_payment_methods_events -> billing_payment_methods (aggregate_id));
joinable!(billing_profiles -> kernel_accounts (account_id));
joinable!(billing_profiles_events -> billing_profiles (aggregate_id));
joinable!(billing_subscriptions -> billing_profiles (billing_profile_id));
joinable!(billing_subscriptions_events -> billing_subscriptions (aggregate_id));
joinable!(bitflow_downloads -> kernel_accounts (owner_id));
joinable!(bitflow_downloads_events -> bitflow_downloads (aggregate_id));
joinable!(bitflow_profiles -> drive_files (download_folder_id));
joinable!(bitflow_profiles -> kernel_accounts (account_id));
joinable!(bitflow_profiles_events -> bitflow_profiles (aggregate_id));
joinable!(calendar_events -> kernel_accounts (owner_id));
joinable!(calendar_events_events -> calendar_events (aggregate_id));
joinable!(contacts_contacts -> kernel_accounts (owner_id));
joinable!(contacts_contacts_events -> contacts_contacts (aggregate_id));
joinable!(drive_files -> kernel_accounts (owner_id));
joinable!(drive_files_events -> drive_files (aggregate_id));
joinable!(drive_profiles -> drive_files (home_id));
joinable!(drive_profiles -> kernel_accounts (account_id));
joinable!(drive_profiles_events -> drive_profiles (aggregate_id));
joinable!(drive_uploads -> drive_files (parent_id));
joinable!(drive_uploads -> kernel_accounts (owner_id));
joinable!(drive_uploads_events -> drive_uploads (aggregate_id));
joinable!(gallery_albums -> kernel_accounts (owner_id));
joinable!(gallery_albums_events -> gallery_albums (aggregate_id));
joinable!(gallery_albums_files -> drive_files (file_id));
joinable!(gallery_albums_files -> gallery_albums (album_id));
joinable!(kernel_accounts_events -> kernel_accounts (aggregate_id));
joinable!(kernel_pending_emails -> kernel_accounts (account_id));
joinable!(kernel_pending_emails_events -> kernel_pending_emails (aggregate_id));
joinable!(kernel_sessions -> kernel_accounts (account_id));
joinable!(kernel_sessions_events -> kernel_sessions (aggregate_id));
joinable!(music_playlists -> kernel_accounts (owner_id));
joinable!(music_playlists_events -> music_playlists (aggregate_id));
joinable!(music_playlists_files -> drive_files (file_id));
joinable!(music_playlists_files -> music_playlists (playlist_id));
joinable!(notes_notes -> kernel_accounts (owner_id));
joinable!(notes_notes_events -> notes_notes (aggregate_id));
joinable!(phaser_reports -> phaser_scans (scan_id));
joinable!(phaser_reports_events -> phaser_reports (aggregate_id));
joinable!(phaser_scans -> kernel_accounts (owner_id));
joinable!(phaser_scans_events -> phaser_scans (aggregate_id));

allow_tables_to_appear_in_same_query!(
    billing_invoices,
    billing_invoices_events,
    billing_payment_methods,
    billing_payment_methods_events,
    billing_profiles,
    billing_profiles_events,
    billing_subscriptions,
    billing_subscriptions_events,
    bitflow_downloads,
    bitflow_downloads_events,
    bitflow_profiles,
    bitflow_profiles_events,
    calendar_events,
    calendar_events_events,
    contacts_contacts,
    contacts_contacts_events,
    drive_files,
    drive_files_events,
    drive_profiles,
    drive_profiles_events,
    drive_uploads,
    drive_uploads_events,
    gallery_albums,
    gallery_albums_events,
    gallery_albums_files,
    kernel_accounts,
    kernel_accounts_events,
    kernel_pending_accounts,
    kernel_pending_accounts_events,
    kernel_pending_emails,
    kernel_pending_emails_events,
    kernel_sessions,
    kernel_sessions_events,
    music_playlists,
    music_playlists_events,
    music_playlists_files,
    notes_notes,
    notes_notes_events,
    phaser_reports,
    phaser_reports_events,
    phaser_scans,
    phaser_scans_events,
);
