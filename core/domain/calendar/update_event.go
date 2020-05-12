package calendar

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/objects"
)

func UpdateEvent(event objects.Object) (objects.Object, error) {
	var err error

	ev, err := ObjectToEvent(&event)
	if err != nil {
		return event, err
	}
	if err = validateUpdateEvent(ev); err != nil {
		return event, err
	}

	event.UpdatedAt = time.Now().UTC()
	err = objects.SaveObject(context.Background(), nil, &event)

	return event, err
}

func validateUpdateEvent(params *Event) error {
	if err := valdiateEventDates(params.StartAt, params.EndAt); err != nil {
		return err
	}
	return nil
}
