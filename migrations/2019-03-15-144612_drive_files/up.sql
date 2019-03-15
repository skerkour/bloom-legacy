-- Your SQL goes here
CREATE TABLE drive_files (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    name TEXT NOT NULL,
    type TEXT NOT NULL,
    size BIGINT NOT NULL,
    parent_id UUID REFERENCES drive_files (id),
    owner_id UUID NOT NULL REFERENCES accounts (id),

    md5 TEXT,
    sha1 TEXT,
    sha256 TEXT,
    sha512 TEXT,
    removed_at TIMESTAMP WITH TIME ZONE,

    PRIMARY KEY(id)
);

CREATE TABLE drive_files_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL,
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
