package groups

import (
	"context"

	"gitlab.com/bloom42/gobox/uuid"
)

// Service is the application service for the groups subdomain
type Service interface {
	// Commands
	AcceptInvitation(ctx context.Context, params AcceptInvitationParams) (Group, error)
	CancelInvitation(ctx context.Context, invitationID uuid.UUID) error
	DeclineInvitation(ctx context.Context, invitationID uuid.UUID) error
	InviteUser(ctx context.Context, params InviteUserParams) (Group, error)
	QuitGroup(ctx context.Context, groupID uuid.UUID) error
	RemoveMembers(ctx context.Context, params RemoveMembersParams) (Group, error)
	UpdateGroup(ctx context.Context, params UpdateGroupParams) (Group, error)
	DeleteGroup(ctx context.Context, groupID uuid.UUID) error
	CreateGroup(ctx context.Context, params CreateGroupParams) (Group, error)

	// Queries
	FindAllGroups(ctx context.Context) ([]Group, error)
	FindGroupById(ctx context.Context, groupID uuid.UUID) (Group, error)
	FindInvitationsForUser(ctx context.Context, userID uuid.UUID) ([]UserInvitation, error)
	FindGroupsForUser(ctx context.Context, userID uuid.UUID) ([]Group, error)
	GroupMembers(ctx context.Context, groupID uuid.UUID) ([]Member, error)
	FindInvitationsForGroup(ctx context.Context, groupID uuid.UUID) ([]GroupInvitation, error)
	Membership(ctx context.Context, groupID uuid.UUID) (Membership, error)
}

type AcceptInvitationParams struct {
	InvitationID       uuid.UUID
	EncryptedMasterKey []byte
	MasterKeyNonce     []byte
}

type InviteUserParams struct {
	GroupID            uuid.UUID
	Username           string
	EphemeralPublicKey []byte
	Signature          []byte
	EncryptedMasterKey []byte
}

type RemoveMembersParams struct {
	GroupID   uuid.UUID
	Usernames []string
}

// UpdateGroupParams are the parameters for UpdateGroup
type UpdateGroupParams struct {
	ID          uuid.UUID
	Name        *string
	Description *string
}

// CreateGroupParams are parameters required to create a group
type CreateGroupParams struct {
	Name               string
	Description        string
	EncryptedMasterKey []byte
	MasterKeyNonce     []byte
}
