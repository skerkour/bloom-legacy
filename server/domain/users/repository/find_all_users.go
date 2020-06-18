package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *UsersRepository) FindAllUsers(ctx context.Context, db db.Queryer) (ret []users.User, err error) {
	ret = []users.User{}
	query := "SELECT * FROM users ORDER BY created_at"

	err = db.Select(ctx, &ret, query)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.FindAllUsers: finding users"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
	}
	return
}
