package objects

import (
	"context"
	"database/sql"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
)

func FindOutOfSyncObjectByID(ctx context.Context, tx *sqlx.Tx, id []byte) (*StoredObject, error) {
	ret := &StoredObject{}
	var err error

	query := "SELECT * FROM objects WHERE out_of_sync = ? AND id = ?"
	if tx == nil {
		err = db.DB.Get(&ret, query, true, id)
	} else {
		err = tx.Get(&ret, query, true, id)
	}
	if err == sql.ErrNoRows {
		return nil, nil
	}

	return ret, err
}
