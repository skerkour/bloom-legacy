package calendar

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/objects"
	"gitlab.com/bloom42/bloom/core/messages"
)

func CreateEvent(params messages.CalendarCreateEventParams) (*objects.Object, error) {
	var err error
	var ret *objects.Object

	if err = validateCreateEvent(params); err != nil {
		return ret, err
	}

	now := time.Now().UTC()
	event := Event{
		Title:       params.Title,
		Description: params.Description,
		StartAt:     params.StartAt,
		EndAt:       params.EndAt,
	}

	id, err := objects.GenerateObjectID([]byte(kernel.Me.Username))
	if err != nil {
		return ret, err
	}

	ret, err = objects.ToObject(id, EVENT_TYPE, now, now, nil, true, &event)
	if err != nil {
		return ret, err
	}

	err = objects.SaveObject(context.Background(), nil, ret)

	// request sync
	objects.SyncChan <- true

	return ret, err
}

func validateCreateEvent(params messages.CalendarCreateEventParams) error {
	if err := valdiateEventDates(params.StartAt, params.EndAt); err != nil {
		return err
	}
	return nil
}
