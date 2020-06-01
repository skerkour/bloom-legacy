package calendar

import (
	"context"
	"encoding/json"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/objects"
	"gitlab.com/bloom42/bloom/core/messages"
)

func ListEvents(params messages.CalendarListEventsParams) (Events, error) {
	var err error
	ret := Events{Events: []objects.Object{}}
	now := time.Now().UTC()
	startOfMonth := time.Date(now.Year(), now.Month(), 1, 0, 0, 0, 0, time.Local)
	var startAt time.Time
	var endAt time.Time

	if params.StartAt == nil {
		startAt = startOfMonth
	} else {
		startAt = *params.StartAt
	}
	if params.EndAt == nil {
		// end of current month
		endAt = startOfMonth.AddDate(0, 1, 0).Add(time.Nanosecond * -1)
	} else {
		endAt = *params.EndAt
	}

	objects, err := objects.FindObjectsByType(context.Background(), nil, kernel.OBJECT_TYPE_CALENDAR_EVENT, nil)
	if err != nil {
		return ret, err
	}

	for _, obj := range objects {
		var event Event

		err = json.Unmarshal(obj.Data, &event)
		if err != nil {
			return ret, err
		}
		if event.StartAt.After(startAt) && event.StartAt.Before(endAt) ||
			event.EndAt.After(startAt) && event.EndAt.Before(endAt) {
			ret.Events = append(ret.Events, obj)
		}
	}

	// bloom_validators::calendar::event_dates(start_at, end_at)?;

	// query := `
	// SELECT id, created_at, updated_at, title, description, start_at, end_at FROM calendar_events
	// WHERE
	// 	(start_at BETWEEN $1 AND $2)
	// 	OR (end_at BETWEEN $1 AND $2)`
	// err = db.DB.Select(&ret.Events, query, params.StartAt, params.EndAt)

	return ret, err
}
