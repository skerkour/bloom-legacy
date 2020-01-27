package calendar

import (
	"time"

	"gitlab.com/bloom42/bloom/core/db"
)

func UpdateEvent(event Event) (Event, error) {
	if err := validateUpdateEvent(event); err != nil {
		return event, err
	}

	event.UpdatedAt = time.Now().UTC()

	stmt, err := db.DB.Prepare(`
		UPDATE calendar_events SET
			updated_at = ?,
			title = ?,
			description = ?,
			start_at = ?,
			end_at = ?
		WHERE id = ?
	`)
	if err != nil {
		return event, err
	}
	defer stmt.Close()

	_, err = stmt.Exec(&event.UpdatedAt, &event.Title, &event.Description, &event.StartAt, &event.EndAt, &event.ID)

	return event, err
}

func validateUpdateEvent(params Event) error {
	if err := valdiateEventDates(params.StartAt, params.EndAt); err != nil {
		return err
	}
	return nil
}
