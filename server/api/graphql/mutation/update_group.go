package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

// UpdateGroup is used by groups' admins to update the group info
func (resolver *Resolver) UpdateGroup(ctx context.Context, input model.GroupInput) (ret *model.Group, err error) {
	params := groups.UpdateGroupParams{
		ID:          input.ID,
		Name:        input.Name,
		Description: input.Description,
	}
	group, err := resolver.groupsService.UpdateGroup(ctx, params)
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
	return ret, nil
}
