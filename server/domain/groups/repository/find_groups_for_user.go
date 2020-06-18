package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) FindGroupsForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) (ret []groups.Group, err error) {
	ret = []groups.Group{}
	query := `SELECT groups.* FROM groups
	INNER JOIN groups_members ON groups_members.group_id = groups.id
	WHERE groups_members.user_id = $1`

	err = db.Select(ctx, &ret, query)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.FindGroupsForUser: finding groups"
		logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", userID))
		err = errors.Internal(errMessage, err)
	}
	return
}
