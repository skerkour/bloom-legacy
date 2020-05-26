package objects

import (
	"context"
	"encoding/base64"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
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
	if err != nil {
		log.Debug("Error saving object", rz.Err(err), rz.String("object.id", base64.StdEncoding.EncodeToString(object.ID)))

	}

	return err
}
