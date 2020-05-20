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
	StartAt     time.Time `json:"startAt"`
	EndAt       time.Time `json:"endAt"`
}

func ObjectToEvent(object *objects.Object) (ret *Event, err error) {
	ret = &Event{}
	err = json.Unmarshal(object.Data, ret)
	return
}
