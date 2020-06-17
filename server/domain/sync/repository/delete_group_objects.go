package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *SyncRepository) DeleteGroupObjects(ctx context.Context, db db.Queryer, groupID uuid.UUID) (err error) {
	query := "DELETE FROM objects WHERE group_id = $1"

	_, err = db.Exec(ctx, query, groupID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "sync.DeleteGroupObjects: deleting object"
		logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", groupID))
		err = errors.Internal(errMessage, err)
	}
	return
}
