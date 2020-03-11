package groups

import (
	"time"
)

type Group struct {
	ID          string    `json:"id" db:"id"`
	CreatedAt   time.Time `json:"createdAt" db:"created_at"`
	Name        string    `json:"name" db:"name"`
	Description string    `json:"description" db:"description"`
	AvatarURL   *string   `json:"avatarUrl" db:"avatar_url"`
}
