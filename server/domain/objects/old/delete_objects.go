package objects

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func DeleteGroupObjects(ctx context.Context, tx *sqlx.Tx, groupID uuid.UUID) (err error) {
	logger := rz.FromCtx(ctx)

	queryDeleteObjects := "DELETE FROM objects WHERE group_id = $1"
	_, err = tx.Exec(queryDeleteObjects, groupID)
	if err != nil {
		logger.Error("objects.DeleteGroupObjects: deleting objects", rz.Err(err))
		err = NewError(ErrorInternal)
		return
	}

	return
}
