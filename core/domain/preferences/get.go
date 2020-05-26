package preferences

import (
	"context"
	"database/sql"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
)

// Get return the value for `key` from the `preferences` table
func Get(ctx context.Context, tx *sqlx.Tx, key string) (*string, error) {
	var err error
	var value string

	query := "SELECT value FROM preferences WHERE key = ?"
	if tx == nil {
		err = db.DB.Get(&value, query, key)
	} else {
		err = tx.Get(&value, query, key)
	}
	if err == sql.ErrNoRows {
		return nil, nil
	}

	if err != nil {
		log.Debug("Error getting preference", rz.Err(err), rz.String("key", key))
	}

	return &value, err
}
