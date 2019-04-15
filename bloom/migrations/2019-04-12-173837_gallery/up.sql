-- Your SQL goes here
CREATE TABLE gallery_albums (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    version BIGINT NOT NULL,

    name TEXT NOT NULL,

    owner_id UUID NOT NULL REFERENCES kernel_accounts(id),

    PRIMARY KEY(id)
);

CREATE TABLE gallery_albums_events (
    id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregate_id UUID NOT NULL REFERENCES gallery_albums (id),
    data JSONB NOT NULL,
    metadata JSONB NOT NULL,

    PRIMARY KEY(id)
);


CREATE TABLE gallery_albums_items (
    id UUID NOT NULL,

    album_id UUID NOT NULL REFERENCES gallery_albums(id),
    file_id UUID NOT NULL REFERENCES drive_files(id),

     PRIMARY KEY(id)
);
