package groups

import (
	"time"
)

// Membership is the Go struct representing the `groups_members` table
type Membership struct {
	JoinedAt  time.Time `json:"joined_at" db:"joined_at"`
	GroupID   string    `json:"group_id" db:"group_id"`
	UserID    string    `json:"user_id" db:"user_id"`
	Role      string    `json:"role" db:"role"`
	InviterID string    `json:"inviter_id" db:"inviter_id"`
}
