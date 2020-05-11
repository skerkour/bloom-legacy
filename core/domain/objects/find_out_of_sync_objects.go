package objects

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
)

func FindOutOfSyncObjects(ctx context.Context, tx *sqlx.Tx) ([]StoredObject, error) {
	ret := []StoredObject{}
	var err error

	query := "SELECT * FROM objects WHERE out_of_sync = ?"
	if tx == nil {
		err = db.DB.Select(&ret, query, true)
	} else {
		err = tx.Select(&ret, query, true)
	}
	return ret, err
}
