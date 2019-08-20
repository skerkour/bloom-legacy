-- Your SQL goes here
CREATE TABLE kernel_accounts (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    avatar_url TEXT NOT NULL,
    email TEXT NOT NULL,
    first_name TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL,
    last_name TEXT NOT NULL,
    password TEXT NOT NULL,
    password_reset_id UUID,
    password_reset_token TEXT,
    username TEXT UNIQUE NOT NULL,

    PRIMARY KEY(id)
);

CREATE UNIQUE INDEX idx_kernel_accounts_username ON kernel_accounts (username);

CREATE TABLE kernel_accounts_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES kernel_accounts (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
