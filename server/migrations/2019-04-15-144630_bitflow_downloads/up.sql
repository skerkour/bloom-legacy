-- Your SQL goes here
-- CREATE TYPE bitflow_download_state AS ENUM ('QUEUED', 'DOWNLOADING', 'COMPLETED', 'FAILED');

CREATE TABLE bitflow_downloads (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    error TEXT,
    name TEXT NOT NULL,
    progress INT NOT NULL,
    removed_at TIMESTAMP WITH TIME ZONE,
    status TEXT NOT NULL,
    url JSONB NOT NULL,

    owner_id UUID NOT NULL REFERENCES kernel_accounts(id),

    PRIMARY KEY(id)
);

CREATE TABLE bitflow_downloads_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES bitflow_downloads(id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
