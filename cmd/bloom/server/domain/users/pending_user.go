package users

import (
	"time"
)

type PendingUser struct {
	ID                   string     `db:"id"`
	CreatedAt            time.Time  `db:"created_at"`
	UpdatedAt            time.Time  `db:"updated_at"`
	Email                string     `db:"email"`
	DisplayName          string     `db:"display_name"`
	VerificationCodeHash string     `db:"verification_code_hash"`
	FailedVerifications  int64      `db:"failed_verifications"`
	VerifiedAt           *time.Time `db:"verified_at"`
}
