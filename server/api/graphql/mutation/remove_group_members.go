package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

// RemoveGroupMembers is used by groups' admins to remove members from a group
func (resolver *Resolver) RemoveGroupMembers(ctx context.Context, input model.RemoveGroupMembersInput) (ret *model.Group, err error) {
	params := groups.RemoveMembersParams{
		GroupID:   input.GroupID,
		Usernames: input.Members,
	}
	group, err := resolver.groupsService.RemoveMembers(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.Group{
		ID:          &group.ID,
		Name:        group.Name,
		Description: group.Description,
		CreatedAt:   &group.CreatedAt,
	}
	return
}
