package groups

import (
	"gitlab.com/bloom42/bloom/core/db"
)

func FindGroups() (Groups, error) {
	ret := Groups{Groups: []Group{}}

	query := "SELECT * FROM groups ORDER BY created_at ASC"
	err := db.DB.Select(&ret.Groups, query)

	return ret, err
}
