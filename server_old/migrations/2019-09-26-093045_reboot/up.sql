-- Your SQL goes here

CREATE TABLE accounts (
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
CREATE UNIQUE INDEX idx_accounts_username ON accounts (username);


CREATE TABLE sessions (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,

    token_hash TEXT NOT NULL,
    ip TEXT NOT NULL,
    user_agent TEXT NOT NULL,

    PRIMARY KEY(id)
);
CREATE INDEX idx_sessions_account_id ON sessions (account_id);


CREATE TABLE pending_accounts (
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


CREATE TABLE deleted_usernames (
    username TEXT NOT NULL,

    PRIMARY KEY(username)
);
CREATE INDEX idx_deleted_usernames_username ON deleted_usernames (username);
