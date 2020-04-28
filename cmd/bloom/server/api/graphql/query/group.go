package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
	"gitlab.com/bloom42/lily/uuid"
)

// Group find a group
func (r *Resolver) Group(ctx context.Context, groupID uuid.UUID) (ret *model.Group, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		err = gqlerrors.AuthenticationRequired()
		return
	}

	err = groups.CheckUserIsGroupMemberNoTx(ctx, currentUser.ID, groupID)
	if err != nil && !currentUser.IsAdmin {
		err = gqlerrors.New(err)
		return
	}

	group, err := groups.FindGroupById(ctx, nil, groupID, false)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = &model.Group{
		ID:          &group.ID,
		CreatedAt:   &group.CreatedAt,
		Name:        group.Name,
		Description: group.Description,
		AvatarURL:   nil,
	}
	return ret, nil
}
