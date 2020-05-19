package groups

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
)

type Invitation struct {
	ID        uuid.UUID `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	GroupID   uuid.UUID `json:"group_id" db:"group_id"`
	InviteeID uuid.UUID `json:"invitee_id" db:"invitee_id"`
	InviterID uuid.UUID `json:"inviter_id" db:"inviter_id"`
}
