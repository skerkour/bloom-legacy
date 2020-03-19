package preferences

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
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

	return err
}
