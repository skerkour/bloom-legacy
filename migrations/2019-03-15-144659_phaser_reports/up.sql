-- Your SQL goes here
CREATE TABLE phaser_reports (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    scan_id UUID NOT NULL REFERENCES phaser_scans (id),
    profile TEXT NOT NULL,
    targets TEXT[] NOT NULL,
    status TEXT NOT NULL,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    duration BIGINT NOT NULL,
    trigger TEXT NOT NULL,
    high_level_findings BIGINT NOT NULL,
    medium_level_findings BIGINT NOT NULL,
    low_level_findings BIGINT NOT NULL,
    information_findings BIGINT NOT NULL,
    findings JSONB,

    PRIMARY KEY(id)
);


CREATE TABLE phaser_reports_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL,
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
