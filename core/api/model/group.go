package model

import (
	"time"
)

type Group struct {
	ID          *string    `json:"id"`
	CreatedAt   *time.Time `json:"createdAt"`
	Name        string     `json:"name"`
	Description string     `json:"description"`
	AvatarURL   *string    `json:"avatarUrl"`
}
