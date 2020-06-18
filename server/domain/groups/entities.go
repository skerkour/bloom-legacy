package groups

import (
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

type Group struct {
	ID          uuid.UUID `db:"id"`
	CreatedAt   time.Time `db:"created_at"`
	UpdatedAt   time.Time `db:"updated_at"`
	Name        string    `db:"name"`
	Description string    `db:"description"`
	Avatar      *string   `db:"avatar"`
	State       int64     `db:"state"`
}

type UserInvitation struct {
	ID                 uuid.UUID `db:"invitation_id"`
	CreatedAt          time.Time `db:"invitation_created_at"`
	EncryptedMasterKey []byte    `db:"invitation_encrypted_master_key"`
	EphemeralPublicKey []byte    `db:"invitation_ephemeral_public_key"`
	Signature          []byte    `db:"invitation_signature"`

	GroupID          uuid.UUID `db:"group_id"`
	GroupCreatedAt   time.Time `db:"group_created_at"`
	GroupName        string    `db:"group_name"`
	GroupDescription string    `db:"group_description"`

	InviterUsername    string `db:"inviter_username"`
	InviterDisplayName string `db:"inviter_display_name"`
	InviterPublicKey   []byte `db:"inviter_public_key"`
}

// Member is used to list group members
type Member struct {
	users.User
	Role     string    `db:"role"`
	JoinedAt time.Time `db:"joined_at"`
}

type GroupInvitation struct {
	ID                 uuid.UUID `db:"invitation_id"`
	CreatedAt          time.Time `db:"invitation_created_at"`
	GroupID            string    `db:"invitation_group_id"`
	InviterID          string    `db:"inviter_id"`
	InviterAvatar      *string   `db:"inviter_avatar"`
	InviterUsername    string    `db:"inviter_username"`
	InviterDisplayName string    `db:"inviter_display_name"`
	InvitedID          string    `db:"invitee_id"`
	InviteeAvatar      *string   `db:"invitee_avatar"`
	InviteeUsername    string    `db:"invitee_username"`
	InviteeDisplayName string    `db:"invitee_display_name"`
}

// Membership is the Go struct representing the `groups_members` table
type Membership struct {
	JoinedAt           time.Time `db:"joined_at"`
	GroupID            uuid.UUID `db:"group_id"`
	UserID             uuid.UUID `db:"user_id"`
	Role               string    `db:"role"`
	EncryptedMasterKey []byte    `db:"encrypted_master_key"`
	MasterKeyNonce     []byte    `db:"master_key_nonce"`
	InviterID          uuid.UUID `db:"inviter_id"`
}

type Invitation struct {
	ID        uuid.UUID `db:"id"`
	CreatedAt time.Time `db:"created_at"`
	UpdatedAt time.Time `db:"updated_at"`

	EphemeralPublicKey []byte `db:"ephemeral_public_key"`
	Signature          []byte `db:"signature"`
	EncryptedMasterKey []byte `db:"encrypted_master_key"`

	GroupID   uuid.UUID `db:"group_id"`
	InviteeID uuid.UUID `db:"invitee_id"`
	InviterID uuid.UUID `db:"inviter_id"`
}
