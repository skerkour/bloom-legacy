package groups

import (
	"time"

	"gitlab.com/bloom42/lily/uuid"
)

// Membership is the Go struct representing the `groups_members` table
type Membership struct {
	JoinedAt  time.Time `json:"joined_at" db:"joined_at"`
	GroupID   uuid.UUID `json:"group_id" db:"group_id"`
	UserID    uuid.UUID `json:"user_id" db:"user_id"`
	Role      string    `json:"role" db:"role"`
	InviterID uuid.UUID `json:"inviter_id" db:"inviter_id"`
}
