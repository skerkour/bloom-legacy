package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/groups"
)

func (service *GroupsService) UpdateGroup(ctx context.Context, params groups.UpdateGroupParams) (ret groups.Group, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	var newName string
	var newDescription string

	err = service.CheckUserIsGroupAdmin(ctx, service.db, me.ID, params.ID)
	if err != nil {
		return
	}

	ret, err = service.groupsRepo.FindGroupByID(ctx, service.db, params.ID)
	if err != nil {
		return
	}

	if params.Name == nil {
		newName = ret.Name
	} else {
		newName = *params.Name
	}
	if params.Description == nil {
		newDescription = ret.Description
	} else {
		newDescription = *params.Description
	}

	err = validateCreateGroup(newName, newDescription)
	if err != nil {
		return
	}

	ret.UpdatedAt = time.Now().UTC()
	ret.Name = newName
	ret.Description = newDescription
	err = service.groupsRepo.UpdateGroup(ctx, service.db, ret)
	return
}
