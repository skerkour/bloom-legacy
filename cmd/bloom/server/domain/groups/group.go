package groups

import (
	"time"

	"gitlab.com/bloom42/lily/uuid"
)

type Group struct {
	ID          uuid.UUID `json:"id" db:"id"`
	CreatedAt   time.Time `json:"created_at" db:"created_at"`
	UpdatedAt   time.Time `json:"updated_at" db:"updated_at"`
	Name        string    `json:"name" db:"name"`
	Description string    `json:"description" db:"description"`
	AvatardID   *string   `json:"avatar_id" db:"avatar_id"`
	State       int64     `db:"state"`
}
