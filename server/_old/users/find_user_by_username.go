package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
)

// FindUserByUsername returns a user for a given username
// if tx is nil, use the global `db.DB`
func FindUserByUsername(ctx context.Context, tx *sqlx.Tx, username string) (*User, error) {
	ret := &User{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM users WHERE username = $1"
	if tx == nil {
		err = db.DB.Get(ret, queryFind, username)
	} else {
		err = tx.Get(ret, queryFind, username)
	}
	if err != nil {
		logger.Error("users.FindUserByUsername: finding user", rz.Err(err),
			rz.String("users.username", username))
		return ret, NewError(ErrorUserNotFound)
	}

	return ret, err
}
