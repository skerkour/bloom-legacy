package users

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

func FindAllUsers(ctx context.Context) ([]User, error) {
	ret := []User{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM users"
	err = db.DB.Select(&ret, queryFind)
	if err != nil {
		logger.Error("users.FindAllUsers: finding users", rz.Err(err))
		return ret, NewError(ErrorUserNotFound)
	}

	return ret, err
}
