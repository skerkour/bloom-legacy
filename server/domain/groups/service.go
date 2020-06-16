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

	// Queries
	FindAllGroups(ctx context.Context) ([]Group, error)
	FindGroupById(ctx context.Context, groupID uuid.UUID) (Group, error)
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
