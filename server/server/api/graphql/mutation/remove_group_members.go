package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
)

// RemoveGroupMembers is used by groups' admins to remove members from a group
func (r *Resolver) RemoveGroupMembers(ctx context.Context, input model.RemoveGroupMembersInput) (*model.Group, error) {
	var ret *model.Group
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	params := groups.RemoveMembersParams{
		GroupID:   input.GroupID,
		Usernames: input.Members,
	}
	group, err := groups.RemoveMembers(ctx, currentUser, params)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &model.Group{
		ID:          &group.ID,
		Name:        group.Name,
		Description: group.Description,
		CreatedAt:   &group.CreatedAt,
	}
	return ret, nil
}
