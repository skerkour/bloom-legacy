package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/sync"
	"gitlab.com/bloom42/gobox/uuid"
)

// Group find a group
func (r *Resolver) Group(ctx context.Context, groupID uuid.UUID) (ret *model.Group, err error) {
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

	group, err := groups.FindGroupById(ctx, nil, groupID, false)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	state = sync.EncodeState(group.State)
	ret = &model.Group{
		ID:          &group.ID,
		CreatedAt:   &group.CreatedAt,
		Name:        group.Name,
		Description: group.Description,
		AvatarURL:   nil,
		State:       &state,
	}
	return ret, nil
}
