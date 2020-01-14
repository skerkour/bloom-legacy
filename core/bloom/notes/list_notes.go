package notes

import (
	"gitlab.com/bloom42/bloom/core/bloom/kernel"
	"gitlab.com/bloom42/bloom/core/db"
)

func ListNotes(_ kernel.Empty) (Notes, error) {
	ret := Notes{Notes: []Note{}}

	rows, err := db.DB.Query(`SELECT id, created_at, updated_at, archived_at, title, body, color, is_pinned
	FROM notes WHERE archived_at IS NULL ORDER BY is_pinned DESC, created_at ASC`)
	if err != nil {
		return ret, nil
	}
	defer rows.Close()

	for rows.Next() {
		var note Note
		err := rows.Scan(&note.ID, &note.CreatedAt, &note.UpdatedAt, &note.ArchivedAt, &note.Title, &note.Body, &note.Color, &note.IsPinned)
		if err != nil {
			return ret, err
		}
		ret.Notes = append(ret.Notes, note)
	}

	return ret, nil
}
