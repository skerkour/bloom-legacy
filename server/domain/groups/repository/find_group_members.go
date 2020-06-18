package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) FindGroupMembers(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret []groups.Member, err error) {
	ret = []groups.Member{}
	query := `SELECT DISTINCT users.id, users.*, groups_members.role, groups_members.joined_at FROM groups, users
		INNER JOIN groups_members ON groups_members.user_id = users.id
		WHERE groups_members.group_id = $1`

	err = db.Select(ctx, &ret, query)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.FindGroupMembers: finding members"
		logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", groupID))
		err = errors.Internal(errMessage, err)
	}
	return
}
