-- Your SQL goes here
CREATE TABLE drive_profiles (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    user_id UUID NOT NULL REFERENCES kernel_users(id),
    total_space BIGINT NOT NULL,
    used_space BIGINT NOT NULL,
    home_id UUID NOT NULL REFERENCES drive_files (id),

    PRIMARY KEY(id)
);

CREATE TABLE drive_profiles_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES drive_profiles (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);
