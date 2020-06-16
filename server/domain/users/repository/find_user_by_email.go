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

// FindUserByEmail find an user with the given email. returns an error if not found
func (repo *UsersRepository) FindUserByEmail(ctx context.Context, db db.Queryer, email string) (ret users.User, err error) {

	query := `SELECT * FROM users WHERE email = $1`
	err = db.Get(ctx, &ret, query, email)

	if err == nil {
		repo.cache.Set(getUserCacheKey(ret.ID), ret)
	} else if err == sql.ErrNoRows {
		err = errors.NotFound(fmt.Sprintf("User not found for email: %s", email))
	} else {
		logger := log.FromCtx(ctx)
		const errMessage = "users.FindUserByEmail: finding user"
		logger.Error(errMessage, log.Err("error", err), log.String("user.email", email))
		err = errors.Internal(errMessage, err)
	}

	return
}
