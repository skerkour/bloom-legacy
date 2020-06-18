package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) GetGroupAdminsCount(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret int64, err error) {
	query := "SELECT COUNT(*) FROM groups_members WHERE group_id = $1 AND role = $2"

	err = db.Get(ctx, &ret, query, customerID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.GetGroupAdminsCount: getting group admin count"
		logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", groupID))
		err = errors.Internal(errMessage, err)
	}
	return
}
