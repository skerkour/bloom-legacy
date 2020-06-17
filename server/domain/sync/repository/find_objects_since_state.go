package repository

import (
	"context"

	"github.com/gofrs/uuid"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// FindObjectsSinceState find objects since the given state
func (repo *SyncRepository) FindObjectsSinceState(ctx context.Context, db db.Queryer, sinceState int64, userID, groupID *uuid.UUID) (ret []sync.Object, err error) {
	logger := log.FromCtx(ctx)
	ret = []sync.Object{}
	var whereID uuid.UUID

	if userID == nil && groupID == nil {
		errMessage := "user.id and group.id can't be both nil"
		logger.Warn("sync.FindObjectsSinceState: " + errMessage)
		err = errors.InvalidArgument(errMessage)
		return
	}

	if userID != nil && groupID != nil {
		errMessage := "userID and groupID can't be both non null"
		logger.Warn("sync.FindObjectSinceState: " + errMessage)
		err = errors.InvalidArgument(errMessage)
		return
	}

	query := "SELECT * FROM objects WHERE updated_at_state > $1 AND"
	if userID != nil {
		query += " user_id = $2"
		whereID = *userID
	} else {
		query += " group_id = $2"
		whereID = *groupID
	}

	err = db.Select(ctx, &ret, query, sinceState, whereID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "sync.FindObjectsSinceState: finding objects"
		logger.Error(errMessage, log.Err("error", err), log.UUID("whereID", whereID), log.String("query", query))
		err = errors.Internal(errMessage, err)
	}

	return
}
