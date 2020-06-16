package users

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
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

// Session is a user's session. one per authenticated device
type Session struct {
	ID         uuid.UUID `db:"id"`
	CreatedAt  time.Time `db:"created_at"`
	UpdatedAt  time.Time `db:"updated_at"`
	Hash       []byte    `db:"hash"`
	Salt       []byte    `db:"salt"`
	DeviceOS   string    `db:"device_os"`
	DeviceType string    `db:"device_type"`
	UserID     uuid.UUID `db:"user_id"`
}

// SessionDevice is a device to ease users to recognise their sessions
type SessionDevice struct {
	OS   string
	Type string
}

// PendingUser is an entity used before an user account is completed to not fill the users table
// with semi filled entries
type PendingUser struct {
	ID                   uuid.UUID  `db:"id"`
	CreatedAt            time.Time  `db:"created_at"`
	UpdatedAt            time.Time  `db:"updated_at"`
	Email                string     `db:"email"`
	DisplayName          string     `db:"display_name"`
	VerificationCodeHash string     `db:"verification_code_hash"`
	FailedAttempts       int64      `db:"failed_attempts"`
	VerifiedAt           *time.Time `db:"verified_at"`
}

// PendingSession are created when 2fa is actived
type PendingSession struct {
	ID             uuid.UUID `db:"id"`
	CreatedAt      time.Time `db:"created_at"`
	VerifiedAt     time.Time `db:"verified_at"`
	Hash           []byte    `db:"hash"`
	Salt           []byte    `db:"salt"`
	FailedAttempts int64     `db:"failed_attempts"`

	UserID uuid.UUID `db:"user_id"`
}
