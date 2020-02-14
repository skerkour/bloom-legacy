CREATE TABLE groups (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    avatar_id TEXT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,

    PRIMARY KEY(id)
);
CREATE UNIQUE INDEX groups_avatar_id_idx ON groups (avatar_id);


CREATE TABLE groups_members (
    joined_at TIMESTAMP WITH TIME ZONE NOT NULL,
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL
);

CREATE INDEX groups_members_group_id_idx ON groups_members (group_id);
CREATE INDEX groups_members_user_id_idx ON groups_members (user_id);


CREATE TABLE groups_invitations (
    id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    invitee_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    inviter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    PRIMARY KEY(id)
);
CREATE INDEX groups_invitations_group_id_idx ON groups_invitations (group_id);
CREATE INDEX groups_invitations_invitee_id ON groups_invitations (invitee_id);
