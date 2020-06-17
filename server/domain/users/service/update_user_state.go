package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (service *UsersService) UpdateUserState(ctx context.Context, db db.Queryer, user users.User, newState int64) (err error) {
	return
}
