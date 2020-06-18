package service

import (
	"context"
	"strings"

	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (service *UsersService) FindUserByUsername(ctx context.Context, username string) (user users.User, err error) {
	_, err = service.Me(ctx)
	if err != nil {
		return
	}

	username = strings.ToLower(username)
	username = strings.TrimSpace(username)

	user, err = service.usersRepo.FindUserByUsername(ctx, service.db, username)
	return
}
