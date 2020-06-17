package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) FindGroupById(ctx context.Context, groupID uuid.UUID) (ret groups.Group, err error) {
	return
}

/*
	currentUser := apiutil.UserFromCtx(ctx)
	var state string

	if currentUser == nil {
		err = gqlerrors.AuthenticationRequired()
		return
	}

	err = groups.CheckUserIsGroupMember(ctx, nil, currentUser.ID, groupID)
	if err != nil && !currentUser.IsAdmin {
		err = gqlerrors.New(err)
		return
	}
*/
