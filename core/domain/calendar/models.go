package calendar

import (
	"time"
)

type Event struct {
	ID          string    `json:"id" db:"id"`
	CreatedAt   time.Time `json:"createdAt" db:"created_at"`
	UpdatedAt   time.Time `json:"updatedAt" db:"updated_at"`
	Title       string    `json:"title" db:"title"`
	Description string    `json:"description" db:"description"`
	StartAt     time.Time `json:"startAt" db:"start_at"`
	EndAt       time.Time `json:"endAt" db:"end_at"`
}
