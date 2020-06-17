package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (service *UsersService) FindAllUsers(ctx context.Context) (ret []users.User, err error) {
	return
}

// admin required
