package notes

import (
	"time"

	"gitlab.com/bloom42/bloom/core/db"
)

func UpdateNote(note Note) (Note, error) {
	now := time.Now().UTC()

	note.UpdatedAt = now

	stmt, err := db.DB.Prepare(`
	UPDATE notes SET
		updatedAt = ?,
		archivedAt = ?,
		title = ?,
		body = ?,
		color = ?,
		isPinned = ?
	WHERE id = ?
	`)
	if err != nil {
		return note, err
	}
	defer stmt.Close()

	_, err = stmt.Exec(&note.UpdatedAt, &note.ArchivedAt, &note.Title, &note.Body, &note.Color, &note.IsPinned, &note.ID)

	return note, err
}
