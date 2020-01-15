package calendar

import (
	"time"
)

type ListEventsParams struct {
	StartAt *time.Time `json:"start_at"`
	EndAt   *time.Time `json:"end_at"`
}

type Events struct {
	Events []Event `json:"events"`
}

type CreateEventParams struct {
	Title       string    `json:"title"`
	Description string    `json:"description"`
	StartAt     time.Time `json:"start_at"`
	EndAt       time.Time `json:"end_at"`
}

type DeleteEventParams struct {
	ID string `json:"id"`
}
