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

func (repo *GroupsRepository) FindMembershipForUser(ctx context.Context, db db.Queryer, userID, groupID uuid.UUID) (ret groups.Membership, err error) {
	query := "SELECT * FROM groups_members WHERE user_id = $1 AND group_id = $2"

	err = db.Get(ctx, &ret, query, userID, groupID)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Not member of group")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "groups.FindMembershipForUser: finding membership"
			logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", userID), log.UUID("group.id", groupID))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
