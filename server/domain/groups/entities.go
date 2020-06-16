package groups

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
)

type Group struct {
	ID          uuid.UUID `db:"id"`
	CreatedAt   time.Time `db:"created_at"`
	UpdatedAt   time.Time `db:"updated_at"`
	Name        string    `db:"name"`
	Description string    `db:"description"`
	AvatardID   *string   `db:"avatar_id"`
	State       int64     `db:"state"`
}
