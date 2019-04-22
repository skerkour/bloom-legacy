-- Your SQL goes here
CREATE TABLE music_playlists (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    name TEXT NOT NULL,

    owner_id UUID NOT NULL REFERENCES kernel_accounts(id),

    PRIMARY KEY(id)
);

CREATE TABLE music_playlists_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES music_playlists (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);


CREATE TABLE music_playlists_files (
    id UUID NOT NULL,

    playlist_id UUID NOT NULL REFERENCES music_playlists(id),
    file_id UUID NOT NULL REFERENCES drive_files(id),

     PRIMARY KEY(id)
);
