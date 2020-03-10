package notes

import (
	"time"

	"github.com/google/uuid"
	"gitlab.com/bloom42/bloom/core/db"
)

func CreateNote(params CreateNoteParams) (Note, error) {
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

	stmt, err := db.DB.Prepare(`INSERT INTO notes (id, created_at, updated_at, archived_at, title,
		body, color, is_pinned)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return note, err
	}
	defer stmt.Close()

	_, err = stmt.Exec(&note.ID, &note.CreatedAt, &note.UpdatedAt, &note.ArchivedAt, &note.Title, &note.Body, &note.Color, &note.IsPinned)

	return note, err
}
