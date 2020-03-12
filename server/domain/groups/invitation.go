package groups

import (
	"time"
)

type Invitation struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	GroupID   string `json:"group_id" db:"group_id"`
	InviteeID string `json:"invitee_id" db:"invitee_id"`
	InviterID string `json:"inviter_id" db:"inviter_id"`
}
