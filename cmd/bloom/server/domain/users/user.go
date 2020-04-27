package users

import (
	"time"

	"gitlab.com/bloom42/lily/uuid"
)

// User represents an user
type User struct {
	ID                  uuid.UUID  `db:"id"`
	CreatedAt           time.Time  `db:"created_at"`
	UpdatedAt           time.Time  `db:"updated_at"`
	AvatardID           *string    `db:"avatar_id"`
	Email               string     `db:"email"`
	DisplayName         string     `db:"display_name"`
	Username            string     `db:"username"`
	AuthKeyHash         string     `db:"auth_key_hash"`
	Bio                 string     `db:"bio"`
	FirstName           string     `db:"first_name"`
	LastName            string     `db:"last_name"`
	IsAdmin             bool       `db:"is_admin"`
	DisabledAt          *time.Time `db:"disabled_at"`
	PublicKey           []byte     `db:"public_key"`
	EncryptedPrivateKey []byte     `db:"encrypted_private_key"`
	PrivateKeyNonce     []byte     `db:"private_key_nonce"`
	EncryptedMasterKey  []byte     `db:"encrypted_master_key"`
	MasterKeyNonce      []byte     `db:"master_key_nonce"`
	State               int64      `db:"state"`
	TwoFASecret         *[]byte    `db:"two_fa_secret"`
}
