package calendar

import (
	"time"
)

type Events struct {
	Events []Event `json:"events"`
}

type Event struct {
	Title       string    `json:"title" db:"title"`
	Description string    `json:"description" db:"description"`
	StartAt     time.Time `json:"startAt" db:"start_at"`
	EndAt       time.Time `json:"endAt" db:"end_at"`
}
