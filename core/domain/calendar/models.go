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
	Title       string    `json:"title"`
	Location    string    `json:"location"`
	Description string    `json:"description"`
	Status      string    `json:"status"` // confirmed, tentative, cancelled
	StartAt     time.Time `json:"startAt"`
	EndAt       time.Time `json:"endAt"`
}

func ObjectToEvent(object *objects.Object) (ret *Event, err error) {
	ret = &Event{}
	err = json.Unmarshal(object.Data, ret)
	return
}
