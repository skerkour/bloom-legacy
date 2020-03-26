package users

import (
	"time"
)

type PendingUser struct {
	ID                   string    `json:"id" db:"id"`
	CreatedAt            time.Time `json:"created_at" db:"created_at"`
	UpdatedAt            time.Time `json:"updated_at" db:"updated_at"`
	Email                string    `json:"email" db:"email"`
	DisplayName          string    `json:"display_name" db:"display_name"`
	VerificationCodeHash string    `json:"verification_code_hash" db:"verification_code_hash"`
	FailedVerifications  int64     `json:"failed_verifications" db:"failed_verifications"`
	Verified             bool      `json:"verified" db:"verified"`
}

type UpdateProfileInput struct {
	ID          *string `json:"id"`
	DisplayName *string `json:"displayName"`
	Bio         *string `json:"bio"`
	FirstName   *string `json:"firstName"`
	LastName    *string `json:"lastName"`
}

type DeletedUsername struct {
	Username string `json:"username" db:"username"`
}
