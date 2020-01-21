package calendar

import (
	"time"

	"github.com/google/uuid"
	"gitlab.com/bloom42/bloom/core/db"
)

func CreateEvent(params CreateEventParams) (Event, error) {
	if err := validateCreateEvent(params); err != nil {
		return Event{}, err
	}

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

	_, err := db.DB.Exec(`INSERT INTO calendar_events (id, created_at, updated_at, title, description, start_at, end_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)`, &event.ID, &event.CreatedAt, &event.UpdatedAt, &event.Title, &event.Description, &event.StartAt, &event.EndAt)

	return event, err
}

func validateCreateEvent(params CreateEventParams) error {
	if err := valdiateEventDates(params.StartAt, params.EndAt); err != nil {
		return err
	}
	return nil
}
