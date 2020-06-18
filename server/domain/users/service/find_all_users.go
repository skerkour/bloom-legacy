package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (service *UsersService) FindAllUsers(ctx context.Context) (ret []users.User, err error) {
	ret = []users.User{}
	me, err := service.Me(ctx)
	if err != nil {
		return
	}

	if !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	ret, err = service.usersRepo.FindAllUsers(ctx, service.db)
	return
}
