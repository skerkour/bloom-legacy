CREATE TABLE account_sessions (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    device JSONB NOT NULL,
    ip TEXT NOT NULL,
    location JSONB,
    token TEXT NOT NULL,

    account_id UUID NOT NULL REFERENCES account_accounts (id),

    PRIMARY KEY(id)
);

CREATE TABLE account_sessions_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL,
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
