package users

import (
	"time"

	"gitlab.com/bloom42/lily/uuid"
)

// PendingSession are created when 2fa is actived
type PendingSession struct {
	ID         uuid.UUID `db:"id"`
	CreatedAt  time.Time `db:"created_at"`
	VerifiedAt time.Time `db:"verified_at"`
	TokenHash  []byte    `db:"token_hash"`
	Salt       []byte    `db:"salt"`
}
