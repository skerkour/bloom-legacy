package groups

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
)

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
