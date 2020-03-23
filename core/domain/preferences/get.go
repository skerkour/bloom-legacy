package preferences

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
)

// Get return the value for `key` from the `preferences` table
func Get(ctx context.Context, tx *sqlx.Tx, key string) (string, error) {
	var err error
	var value string

	query := "SELECT value FROM preferences WHERE key = ?"
	if tx == nil {
		err = db.DB.Get(&value, query, key)
	} else {
		err = tx.Get(&value, query, key)
	}

	return value, err
}
