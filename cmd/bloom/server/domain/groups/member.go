package groups

import (
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
)

type Member struct {
	users.User
	Role               string    `db:"role"`
	EncryptedMasterKey []byte    `db:"encrypted_master_key"`
	MasterKeyNonce     []byte    `db:"master_key_nonce"`
	JoinedAt           time.Time `db:"joined_at"`
}
