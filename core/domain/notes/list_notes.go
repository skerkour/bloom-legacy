package notes

import (
	"gitlab.com/bloom42/bloom/core/db"
)

func ListNotes() (Notes, error) {
	ret := Notes{Notes: []Note{}}

	query := "SELECT * FROM notes WHERE archived_at IS NULL ORDER BY is_pinned DESC, created_at ASC"
	err := db.DB.Select(&ret.Notes, query)
	if err != nil {
		return ret, err
	}

	return ret, nil
}
