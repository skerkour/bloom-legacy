package calendar

import (
	"time"

	"gitlab.com/bloom42/bloom/core/db"
)

func ListEvents(params ListEventsParams) (Events, error) {
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

	rows, err := db.DB.Query(`
	SELECT id, created_at, updated_at, title, description, start_at, end_at FROM calendar_events
	WHERE
		(start_at BETWEEN $1 AND $2)
		OR (end_at BETWEEN $1 AND $2)`, params.StartAt, params.EndAt)
	if err != nil {
		return ret, err
	}
	defer rows.Close()

	for rows.Next() {
		var event Event
		err := rows.Scan(&event.ID, &event.CreatedAt, &event.UpdatedAt, &event.Title, &event.Description, &event.StartAt, &event.EndAt)
		if err != nil {
			return ret, err
		}
		ret.Events = append(ret.Events, event)
	}

	return ret, nil
}
