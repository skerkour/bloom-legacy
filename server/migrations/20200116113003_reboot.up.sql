-- Your SQL goes here

CREATE TABLE users (
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
CREATE UNIQUE INDEX idx_users_username ON users (username);


CREATE TABLE sessions (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    token_hash TEXT NOT NULL,
    ip TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    device_os TEXT NOT NULL,
    device_type TEXT NOT NULL,

    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,


    PRIMARY KEY(id)
);
CREATE INDEX idx_sessions_user_id ON sessions (user_id);


CREATE TABLE pending_users (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    email TEXT NOT NULL,
    display_name TEXT NOT NULL,

    verification_code_hash TEXT NOT NULL,
    failed_verifications BIGINT NOT NULL,
    verified BOOLEAN NOT NULL,

    PRIMARY KEY(id)
);


CREATE TABLE deleted_usernames (
    username TEXT NOT NULL,

    PRIMARY KEY(username)
);
CREATE INDEX idx_deleted_usernames_username ON deleted_usernames (username);
