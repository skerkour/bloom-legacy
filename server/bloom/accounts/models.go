package accounts

import "time"

type Account struct {
	ID       string `json:"id" db:"id"`
	Username string `json:"username" db:"username"`
}

type PendingAccount struct {
	ID                   string    `json:"id" db:"id"`
	CreatedAt            time.Time `json:"created_at" db:"created_at"`
	UpdatedAt            time.Time `json:"updated_at" db:"updated_at"`
	Email                string    `json:"email" db:"email"`
	DisplayName          string    `json:"display_name" db:"display_name"`
	VerificationCodeHash string    `json:"verification_code_hash" db:"verification_code_hash"`
	Trials               int64     `json:"trials" db:"trials"`
	Verified             bool      `json:"verified" db:"verified"`
}

type Session struct {
	ID string `json:"id" db:"id"`
}

type DeletedUsername struct {
	Username string `json:"username" db:"username"`
}
