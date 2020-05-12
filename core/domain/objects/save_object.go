package objects

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
)

// SaveObject create or replace an existing object
func SaveObject(ctx context.Context, tx *sqlx.Tx, object *Object) error {
	var err error

	query := `INSERT OR REPLACE INTO objects
		(id, created_at, updated_at, type, data, out_of_sync, group_id)
		VALUES (?, ?, ?, ?, ?, ?, ?)`
	args := []interface{}{object.ID, object.CreatedAt, object.UpdatedAt, object.Type,
		object.Data, object.OutOfSync, object.GroupID}
	if tx == nil {
		_, err = db.DB.Exec(query, args...)
	} else {
		_, err = tx.Exec(query, args...)
	}

	return err
}
