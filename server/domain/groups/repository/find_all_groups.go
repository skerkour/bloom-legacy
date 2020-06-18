package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *GroupsRepository) FindAllGroups(ctx context.Context, db db.Queryer) (ret []groups.Group, err error) {
	ret = []groups.Group{}
	query := "SELECT * FROM groups ORDER BY created_at"

	err = db.Select(ctx, &ret, query)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.FindAllGroups: finding groups"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
	}
	return
}
