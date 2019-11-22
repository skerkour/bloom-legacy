table! {
    accounts (id) {
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
    deleted_usernames (username) {
        username -> Text,
    }
}

table! {
    pending_accounts (id) {
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
    sessions (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        account_id -> Uuid,
        token_hash -> Text,
        ip -> Text,
        user_agent -> Text,
    }
}

joinable!(sessions -> accounts (account_id));

allow_tables_to_appear_in_same_query!(accounts, deleted_usernames, pending_accounts, sessions,);
