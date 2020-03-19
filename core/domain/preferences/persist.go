package preferences

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
)

// Persist insert or replace a key/value pair in the `preferences` table
func Persist(ctx context.Context, tx *sqlx.Tx, key, value string) error {
	var err error

	query := "INSERT OR REPLACE INTO preferences (key, value) VALUES (?, ?)"
	if tx == nil {
		_, err = db.DB.Exec(query, key, value)
	} else {
		_, err = tx.Exec(query, key, value)
	}

	return err
}
