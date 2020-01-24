package groups

import (
	"time"
)

type Group struct {
	ID          string    `json:"id" db:"id"`
	CreatedAt   time.Time `json:"created_at" db:"created_at"`
	UpdatedAt   time.Time `json:"updated_at" db:"updated_at"`
	Name        string    `json:"name" db:"name"`
	Description string    `json:"description" db:"description"`
}

type Membership struct {
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	GroupID   string    `json:"group_id" db:"group_id"`
	UserID    string    `json:"user_id" db:"user_id"`
	Role      string    `json:"role" db:"role"`
}

type Invitation struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	GroupID   string `json:"group_id" db:"group_id"`
	InviteeID string `json:"invitee_id" db:"invitee_id"`
}
