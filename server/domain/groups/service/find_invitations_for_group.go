package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) FindInvitationsForGroup(ctx context.Context, groupID uuid.UUID) (ret []groups.GroupInvitation, err error) {
	ret = []groups.GroupInvitation{}
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	err = service.CheckUserIsGroupAdmin(ctx, service.db, me.ID, groupID)
	if err != nil {
		return
	}

	ret, err = service.groupsRepo.FindInvitationsForGroup(ctx, service.db, groupID)
	return
}
