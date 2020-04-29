package calendar

import (
	"time"

	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/messages"
)

func ListEvents(params messages.CalendarListEventsParams) (Events, error) {
	var err error
	ret := Events{Events: []Event{}}
	now := time.Now().UTC()
	startOfMonth := time.Date(now.Year(), now.Month(), 1, 0, 0, 0, 0, time.Local)

	if params.StartAt == nil {
		params.StartAt = &startOfMonth
	}
	if params.EndAt == nil {
		endOfMonth := startOfMonth.AddDate(0, 1, 0).Add(time.Nanosecond * -1)
		params.EndAt = &endOfMonth
	}

	// bloom_validators::calendar::event_dates(start_at, end_at)?;

	query := `
	SELECT id, created_at, updated_at, title, description, start_at, end_at FROM calendar_events
	WHERE
		(start_at BETWEEN $1 AND $2)
		OR (end_at BETWEEN $1 AND $2)`
	err = db.DB.Select(&ret.Events, query, params.StartAt, params.EndAt)

	return ret, err
}
