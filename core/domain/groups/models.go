package groups

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
)

type Groups struct {
	Groups []Group `json:"groups"`
}

type Group struct {
	ID          uuid.UUID `json:"id" db:"id"`
	CreatedAt   time.Time `json:"createdAt" db:"created_at"`
	Name        string    `json:"name" db:"name"`
	Description string    `json:"description" db:"description"`
	AvatarURL   *string   `json:"avatarUrl" db:"avatar_url"`
	State       string    `json:"state" db:"state"`
	MasterKey   []byte    `json:"-" db:"master_key"`
}
