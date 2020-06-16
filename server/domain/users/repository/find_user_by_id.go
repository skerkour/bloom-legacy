package repository

import (
	"context"
	"database/sql"
	"fmt"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

// FindUserByID find an user with the given ID. returns an error if nut found
func (repo *UsersRepository) FindUserByID(ctx context.Context, db db.Queryer, userID uuid.UUID) (ret users.User, err error) {

	if cacheValue, found := repo.cache.Get(getUserCacheKey(userID)); found {
		ret = cacheValue.(users.User)
		return
	}

	query := `SELECT * FROM users WHERE id = $1`
	err = db.Get(ctx, &ret, query, userID)

	if err == nil {
		repo.cache.Set(getUserCacheKey(ret.ID), ret)
	} else if err == sql.ErrNoRows {
		err = errors.NotFound(fmt.Sprintf("User with id: %s not found", userID.String()))
	} else {
		logger := log.FromCtx(ctx)
		const errMessage = "users.FindUserByID: finding user"
		logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", userID))
		err = errors.Internal(errMessage, err)
	}

	return
}
