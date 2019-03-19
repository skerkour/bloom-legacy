-- Your SQL goes here
CREATE TABLE account_accounts (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    avatar_url TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL,
    recovery_id UUID,
    recovery_token TEXT,

    PRIMARY KEY(id)
);

-- CREATE INDEX idx_accounts_username ON accounts (username);

CREATE TABLE account_accounts_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL,
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
