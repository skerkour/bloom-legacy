package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) Membership(ctx context.Context, groupID uuid.UUID) (ret groups.Membership, err error) {
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

	membership, err := groups.FindGroupMasterKey(ctx, nil, *group.ID, currentUser.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}
*/
