-- Your SQL goes here
CREATE TABLE phaser_scans (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    description TEXT NOT NULL,
    last TIMESTAMP WITH TIME ZONE,
    name TEXT NOT NULL,
    profile TEXT NOT NULL,
    schedule TEXT NOT NULL,
    state TEXT NOT NULL,
    targets TEXT[] NOT NULL,

    owner_id UUID NOT NULL REFERENCES kernel_accounts (id),

    PRIMARY KEY(id)
);


CREATE TABLE phaser_scans_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES phaser_scans (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
