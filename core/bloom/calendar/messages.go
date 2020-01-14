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
