package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (service *GroupsService) FindAllGroups(ctx context.Context) (ret []groups.Group, err error) {
	ret = []groups.Group{}
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	if !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	ret, err = service.groupsRepo.FindAllGroups(ctx, service.db)
	return
}
