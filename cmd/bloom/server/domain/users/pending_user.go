package users

import (
	"time"

	"gitlab.com/bloom42/lily/uuid"
)

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
