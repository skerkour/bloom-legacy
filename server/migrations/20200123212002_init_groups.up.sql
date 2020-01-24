CREATE TABLE groups (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    avatar_id TEXT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,

    PRIMARY KEY(id)
);

CREATE TABLE groups_members (
    group_id UUID NOT NULL REFERENCES groups(id),
    user_id UUID NOT NULL REFERENCES users(id),
    role TEXT NOT NULL
);

CREATE TABLE groups_invitations (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    group_id UUID NOT NULL REFERENCES groups(id),
    user_id UUID NOT NULL REFERENCES users(id),

    PRIMARY KEY(id)
);
