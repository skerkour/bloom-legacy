-- Your SQL goes here
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
    url TEXT NOT NULL,

    file_id UUID REFERENCES drive_files(id),
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
