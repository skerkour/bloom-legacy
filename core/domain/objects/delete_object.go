package objects

import (
	"context"
	"encoding/base64"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
)

func DeleteObject(ctx context.Context, tx *sqlx.Tx, objectID []byte) error {
	var err error
	emptyData := []byte("{}")

	query := "UPDATE objects SET data = ?, out_of_sync = ? WHERE id = ?"
	args := []interface{}{emptyData, true, objectID}
	if tx == nil {
		_, err = db.DB.Exec(query, args...)
	} else {
		_, err = tx.Exec(query, args...)
	}
	if err != nil {
		log.Debug("Error deleting object", rz.Err(err),
			rz.String("object.id", base64.StdEncoding.EncodeToString(objectID)))
	}
	return err
}
