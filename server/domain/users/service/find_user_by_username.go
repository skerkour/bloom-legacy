package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (service *UsersService) FindUserByUsername(ctx context.Context, username string) (user users.User, err error) {
	return
}
