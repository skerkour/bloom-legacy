package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func DeleteGroupObjects(ctx context.Context, tx *sqlx.Tx, groupID uuid.UUID) error {
	var err error

	query := "DELETE FROM objects WHERE group_id = ?"
	if tx == nil {
		_, err = db.DB.Exec(query, groupID)
	} else {
		_, err = tx.Exec(query, groupID)
	}
	if err != nil {
		log.Debug("groups.DeleteGroupObjects: Error deleting objects",
			rz.Err(err), rz.String("group.id", groupID.String()))
	}
	return err
}
