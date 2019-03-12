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

allow_tables_to_appear_in_same_query!(
    account_pending_accounts,
    account_pending_accounts_events,
);
