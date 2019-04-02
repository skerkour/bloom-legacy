-- Your SQL goes here
CREATE TABLE drive_upload_sessions (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    version BIGINT NOT NULL,

    file_id UUID NOT NULL,
    file_name TEXT NOT NULL,
    parent_id UUID REFERENCES drive_files (id),
    presigned_url TEXT NOT NULL,

    owner_id UUID NOT NULL REFERENCES kernel_accounts (id),

    PRIMARY KEY(id)
);

CREATE TABLE drive_upload_sessions_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES drive_files (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
