package preferences

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
)

// Delete a key/value pair from the `preferences` table
func Delete(ctx context.Context, tx *sqlx.Tx, key string) error {
	var err error

	query := "DELETE FROM preferences WHERE key = ?"
	if tx == nil {
		_, err = db.DB.Exec(query, key)
	} else {
		_, err = tx.Exec(query, key)
	}

	if err != nil {
		log.Debug("Error deleting preference", rz.Err(err), rz.String("key", key))
	}

	return err
}
