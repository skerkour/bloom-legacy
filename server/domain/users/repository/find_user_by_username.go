package repository

import (
	"context"
	"database/sql"
	"fmt"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// FindUserByUsername find an user with the given username. returns an error if not found
func (repo *UsersRepository) FindUserByUsername(ctx context.Context, db db.Queryer, username string) (ret users.User, err error) {
	query := `SELECT * FROM users WHERE username = $1`

	err = db.Get(ctx, &ret, query, username)
	if err == nil {
		repo.cache.Set(getUserCacheKey(ret.ID), ret)
	} else if err == sql.ErrNoRows {
		err = errors.NotFound(fmt.Sprintf("User not found for username: %s", username))
	} else {
		logger := log.FromCtx(ctx)
		const errMessage = "users.FindUserByUsername: finding user"
		logger.Error(errMessage, log.Err("error", err), log.String("user.username", username))
		err = errors.Internal(errMessage, err)
	}
	return
}
