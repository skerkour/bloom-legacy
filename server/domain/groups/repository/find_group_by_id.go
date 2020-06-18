package repository

import (
	"context"
	"database/sql"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) FindGroupByID(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret groups.Group, err error) {
	query := "SELECT * FROM groups WHERE id = $1"

	err = db.Get(ctx, &ret, query, groupID)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("PlGroupan not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "groups.FindGroupByID: finding group"
			logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", groupID))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
