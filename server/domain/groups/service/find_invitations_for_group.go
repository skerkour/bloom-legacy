package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) FindInvitationsForGroup(ctx context.Context, groupID uuid.UUID) (ret []groups.GroupInvitation, err error) {
	return
}

/*

	currentUser := apiutil.UserFromCtx(ctx)

	if group.ID == nil {
		return ret, PermissionDeniedToAccessField()
	}

	err = groups.CheckUserIsGroupAdmin(ctx, nil, currentUser.ID, *group.ID)
	if err != nil && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	invitations, err := groups.FindGroupInvitations(ctx, nil, *group.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

*/
