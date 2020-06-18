package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) GroupMembers(ctx context.Context, groupID uuid.UUID) (ret []groups.Member, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	err = service.CheckUserIsGroupMember(ctx, service.db, me.ID, groupID)
	if err != nil {
		// if permission denied error, we don't return yet before checking that actor is an instance admin
		if _, ok := err.(*errors.PermissionDeniedError); !ok {
			return
		}
	}
	if err != nil && !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	ret, err = service.groupsRepo.FindGroupMembers(ctx, service.db, groupID)
	return
}
