package users

import "time"

type User struct {
	ID          string     `json:"id" db:"id"`
	CreatedAt   time.Time  `json:"created_at" db:"created_at"`
	UpdatedAt   time.Time  `json:"updated_at" db:"updated_at"`
	AvatardID   *string    `json:"avatar_id" db:"avatar_id"`
	Email       string     `json:"email" db:"email"`
	DisplayName string     `json:"display_name" db:"display_name"`
	Username    string     `json:"username" db:"username"`
	AuthKeyHash string     `json:"-" db:"auth_key_hash"`
	Bio         string     `json:"bio" db:"bio"`
	FirstName   string     `json:"first_name" db:"first_name"`
	LastName    string     `json:"last_name" db:"last_name"`
	IsAdmin     bool       `json:"is_admin" db:"is_admin"`
	DisabledAt  *time.Time `json:"disabled_at" db:"disabled_at"`
}

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

type Session struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`
	UserID    string    `json:"user_id" db:"user_id"`
	TokenHash string    `json:"token_hash" db:"token_hash"`
	IPAddr    string    `json:"ip" db:"ip"`
	UserAgent string    `json:"user_agent" db:"user_agent"`
}

type DeletedUsername struct {
	Username string `json:"username" db:"username"`
}
