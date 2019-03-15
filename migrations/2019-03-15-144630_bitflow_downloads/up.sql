-- Your SQL goes here
CREATE TABLE bitflow_downloads (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    name TEXT NOT NULL,
    status TEXT NOT NULL,
    url TEXT NOT NULL,
    progress INT NOT NULL,
    error TEXT,
    owner_id UUID NOT NULL REFERENCES accounts(id),
    file_id UUID REFERENCES drive_files(id),
    removed_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY(id)
);

CREATE TABLE bitflow_downloads_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL,
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
