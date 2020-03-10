package calendar

import (
	"time"
)

type ListEventsParams struct {
	StartAt *time.Time `json:"startAt"`
	EndAt   *time.Time `json:"endAt"`
}

type Events struct {
	Events []Event `json:"events"`
}

type CreateEventParams struct {
	Title       string    `json:"title"`
	Description string    `json:"description"`
	StartAt     time.Time `json:"startAt"`
	EndAt       time.Time `json:"endAt"`
}

type DeleteEventParams struct {
	ID string `json:"id"`
}
