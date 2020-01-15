package calendar

import (
	"time"

	"gitlab.com/bloom42/bloom/core/db"
)

func UpdateEvent(event Event) (Event, error) {
	// validators
	now := time.Now().UTC()

	event.UpdatedAt = now

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
