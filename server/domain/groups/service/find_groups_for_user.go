package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) FindGroupsForUser(ctx context.Context, userID uuid.UUID) (ret []groups.Group, err error) {
	return
}

/*
	var ret *GroupConnection
	currentUser := apiutil.UserFromCtx(ctx)
	logger := rz.FromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

		groups, err := groups.FindGroupsForUser(ctx, nil, currentUser.ID)

*/
