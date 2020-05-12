package calendar

import (
	"encoding/json"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/objects"
)

type Events struct {
	Events []objects.Object `json:"events"`
}

type Event struct {
	Title       string    `json:"title" db:"title"`
	Description string    `json:"description" db:"description"`
	StartAt     time.Time `json:"startAt" db:"start_at"`
	EndAt       time.Time `json:"endAt" db:"end_at"`
}

func ObjectToEvent(object *objects.Object) (ret *Event, err error) {
	ret = &Event{}
	err = json.Unmarshal(object.Data, ret)
	return
}
