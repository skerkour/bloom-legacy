package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
)

func FindGroups(ctx context.Context, tx *sqlx.Tx) (Groups, error) {
	ret := Groups{Groups: []Group{}}
	var err error

	query := "SELECT * FROM groups ORDER BY created_at ASC"
	if tx == nil {
		err = db.DB.Select(&ret.Groups, query)
	} else {
		err = tx.Select(&ret.Groups, query)
	}

	return ret, err
}
