package notes

import (
	"gitlab.com/bloom42/bloom/core/db"
)

func ListNotes() (Notes, error) {
	ret := Notes{Notes: []Note{}}

	query := "SELECT * FROM notes WHERE archived_at IS NULL ORDER BY updated_at DESC"
	err := db.DB.Select(&ret.Notes, query)
	if err != nil {
		return ret, err
	}

	return ret, nil
}

func ListArchived() (Notes, error) {
	ret := Notes{Notes: []Note{}}

	query := "SELECT * FROM notes WHERE archived_at IS NOT NULL ORDER BY updated_at DESC"
	err := db.DB.Select(&ret.Notes, query)
	if err != nil {
		return ret, err
	}

	return ret, nil
}
