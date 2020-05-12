package objects

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
)

// SaveObject create or replace an existing object
func SaveObject(ctx context.Context, tx *sqlx.Tx, object *StoredObject) error {
	var err error

	query := `INSERT OR REPLACE INTO objects
		(id, type, data, out_of_sync, group_id)
		VALUES (?, ?, ?, ?, ?)`
	if tx == nil {
		_, err = db.DB.Exec(query, object.ID, object.Type, object.Data, object.OutOfSync, object.GroupID)
	} else {
		_, err = tx.Exec(query, object.ID, object.Type, object.Data, object.OutOfSync, object.GroupID)
	}

	return err
}
