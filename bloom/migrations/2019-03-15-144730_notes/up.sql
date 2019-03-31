-- Your SQL goes here
CREATE TABLE notes_notes (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    archived_at TIMESTAMP WITH TIME ZONE,
    body TEXT NOT NULL,
    removed_at TIMESTAMP WITH TIME ZONE,
    title TEXT NOT NULL,

    owner_id UUID NOT NULL REFERENCES kernel_users (id),

    PRIMARY KEY(id)
);

CREATE TABLE notes_notes_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES notes_notes (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
