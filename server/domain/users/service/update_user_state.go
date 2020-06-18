package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (service *UsersService) UpdateUserState(ctx context.Context, db db.Queryer, user users.User, newState int64) (err error) {
	if user.State > newState {
		err = users.ErrCantDowngradeUserState
		return
	}

	user.State = newState
	user.UpdatedAt = time.Now().UTC()
	err = service.usersRepo.UpdateUser(ctx, db, user)
	return
}
