CREATE TABLE kernel_sessions (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    device JSONB NOT NULL,
    ip TEXT NOT NULL,
    location JSONB,
    token TEXT NOT NULL,

    account_id UUID NOT NULL REFERENCES kernel_accounts (id),

    PRIMARY KEY(id)
);

CREATE TABLE kernel_sessions_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES kernel_sessions (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
