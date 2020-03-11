package notes

import (
	"time"

	"gitlab.com/bloom42/bloom/core/db"
)

func UpdateNote(note Note) (Note, error) {
	now := time.Now().UTC()
	var err error

	note.UpdatedAt = now

	query := `
	UPDATE notes SET
		updated_at = ?,
		archived_at = ?,
		title = ?,
		body = ?,
		color = ?,
		is_pinned = ?
	WHERE id = ?
	`
	_, err = db.DB.Exec(query, &note.UpdatedAt, &note.ArchivedAt, &note.Title, &note.Body, &note.Color, &note.IsPinned, &note.ID)

	return note, err
}
