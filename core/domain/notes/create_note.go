package notes

import (
	"time"

	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/lily/uuid"
)

func CreateNote(params messages.CreateNoteParams) (Note, error) {
	var err error
	now := time.Now().UTC()
	uuid := uuid.New()
	note := Note{
		ID:         uuid.String(),
		CreatedAt:  now,
		UpdatedAt:  now,
		ArchivedAt: nil,
		Title:      params.Title,
		Body:       params.Body,
		Color:      params.Color,
		IsPinned:   false,
	}

	query := `INSERT INTO notes (id, created_at, updated_at, archived_at, title,
		body, color, is_pinned)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?)`
	_, err = db.DB.Exec(query, &note.ID, &note.CreatedAt, &note.UpdatedAt, &note.ArchivedAt, &note.Title, &note.Body, &note.Color, &note.IsPinned)

	return note, err
}
