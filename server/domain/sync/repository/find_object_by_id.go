package repository

import (
	"context"
	"database/sql"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// FindObjectByID find an object with the given ID. returns an error if nut found
func (repo *SyncRepository) FindObjectByID(ctx context.Context, db db.Queryer, objectID []byte) (ret sync.Object, err error) {
	query := "SELECT * FROM objects WHERE id = $1"
	err = db.Get(ctx, &ret, query, objectID)

	if err == sql.ErrNoRows {
		err = sync.ErrObjectNotFound
	} else {
		logger := log.FromCtx(ctx)
		const errMessage = "sync.FindObjectByID: finding object"
		logger.Error(errMessage, log.Err("error", err), log.Base64("object.id", objectID))
		err = errors.Internal(errMessage, err)
	}

	return
}
