CREATE TABLE groups (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    avatar_id TEXT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,

    PRIMARY KEY(id)
);
