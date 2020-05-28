package groups

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
)

type Invitation struct {
	ID        uuid.UUID `db:"id"`
	CreatedAt time.Time `db:"created_at"`
	UpdatedAt time.Time `db:"updated_at"`

	EphemeralPublicKey          []byte `db:"ephemeral_public_key"`
	Signature                   []byte `db:"signature"`
	EncryptedMasterKey          []byte `db:"encrypted_master_key"`
	EncryptedMasterKeySignature []byte `db:"encrypted_master_key_signature"`

	GroupID   uuid.UUID `db:"group_id"`
	InviteeID uuid.UUID `db:"invitee_id"`
	InviterID uuid.UUID `db:"inviter_id"`
}
