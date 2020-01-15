package calendar

import (
	"time"

	"github.com/google/uuid"
	"gitlab.com/bloom42/bloom/core/db"
)

func CreateEvent(params CreateEventParams) (Event, error) {
	// bloom_validators::calendar::event_dates(input.start_at, input.end_at)?;
	now := time.Now().UTC()
	uuid := uuid.New()
	event := Event{
		ID:          uuid.String(),
		CreatedAt:   now,
		UpdatedAt:   now,
		Title:       params.Title,
		Description: params.Description,
		StartAt:     params.StartAt,
		EndAt:       params.EndAt,
	}

	stmt, err := db.DB.Prepare(`INSERT INTO calendar_events (id, created_at, updated_at, title, description, start_at, end_at)
		VALUES (?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return event, err
	}
	defer stmt.Close()

	_, err = stmt.Exec(&event.ID, &event.CreatedAt, &event.UpdatedAt, &event.Title, &event.Description, &event.StartAt, &event.EndAt)

	return event, err
}
