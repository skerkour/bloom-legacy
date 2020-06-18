package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) FindInvitationsForUser(ctx context.Context, userID uuid.UUID) (ret []groups.UserInvitation, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	if me.ID != userID && !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	ret, err = service.groupsRepo.FindInvitationsForUser(ctx, service.db, userID)
	return
}
