package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
)

func (r *Resolver) InviteUsersInGroup(ctx context.Context, input model.InviteUsersInGroupInput) (ret *model.Group, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	params := groups.InviteUsersParams{
		GroupID:   input.ID,
		Usernames: input.Users,
	}
	group, err := groups.InviteUsers(ctx, currentUser, params)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = &model.Group{
		ID:          &group.ID,
		CreatedAt:   &group.CreatedAt,
		Name:        group.Name,
		Description: group.Description,
	}

	return
}
