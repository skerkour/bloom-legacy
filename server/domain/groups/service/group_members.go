package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) GroupMembers(ctx context.Context, groupID uuid.UUID) (ret []groups.Member, err error) {
	return
}

/*
	currentUser := apiutil.UserFromCtx(ctx)
	var err error

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupMember(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	members, err := groups.FindGroupMembers(ctx, nil, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

*/
