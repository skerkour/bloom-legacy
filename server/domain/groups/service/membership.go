package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) Membership(ctx context.Context, groupID uuid.UUID) (ret groups.Membership, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	ret, err = service.groupsRepo.FindMembershipForUser(ctx, service.db, me.ID, groupID)
	return
}
