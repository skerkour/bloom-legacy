package notes

import (
	"gitlab.com/bloom42/bloom/core/bloom/kernel"
	"gitlab.com/bloom42/bloom/core/db"
)

func ListArchived(_ kernel.Empty) ([]Note, error) {
	ret := []Note{}

	rows, err := db.DB.Query(`SELECT id, created_at, updated_at, archived_at, title, body, color, is_pinned
		WHERE archived_at IS NOT NULL ORDER BY updated_at DESC`)
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
		ret = append(ret, note)
	}

	return ret, nil
}
