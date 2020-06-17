package groups

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
)

type Group struct {
	ID          uuid.UUID `db:"id"`
	CreatedAt   time.Time `db:"created_at"`
	UpdatedAt   time.Time `db:"updated_at"`
	Name        string    `db:"name"`
	Description string    `db:"description"`
	AvatardID   *string   `db:"avatar_id"`
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
