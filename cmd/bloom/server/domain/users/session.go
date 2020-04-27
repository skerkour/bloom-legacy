package users

import (
	"time"

	"gitlab.com/bloom42/lily/uuid"
)

// Session is a user's session. one per authenticated device
type Session struct {
	ID         uuid.UUID `json:"id" db:"id"`
	CreatedAt  time.Time `json:"created_at" db:"created_at"`
	UpdatedAt  time.Time `json:"updated_at" db:"updated_at"`
	Hash       []byte    `json:"hash" db:"hash"`
	Salt       []byte    `json:"salt" db:"salt"`
	DeviceOS   string    `json:"device_os" db:"device_os"`
	DeviceType string    `json:"device_type" db:"device_type"`
	UserID     uuid.UUID `json:"user_id" db:"user_id"`
}

// SessionDevice is a device to ease users to recognise their sessions
type SessionDevice struct {
	OS   string
	Type string
}
