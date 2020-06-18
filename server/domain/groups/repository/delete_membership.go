package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) DeleteMembership(ctx context.Context, db db.Queryer, userID, groupID uuid.UUID) (err error) {
	query := "DELETE FROM groups_members WHERE user_id = $1 AND group_id = $2"

	_, err = db.Exec(ctx, query, userID, groupID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.DeleteMembership: deleting membership"
		logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", groupID), log.UUID("user.id", userID))
		err = errors.Internal(errMessage, err)
	}
	return
}
