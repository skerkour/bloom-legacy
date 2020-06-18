package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// Groups finds all groups
func (resolver *Resolver) Groups(ctx context.Context) (ret *model.GroupConnection, err error) {
	groups, err := resolver.groupsService.FindAllGroups(ctx)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.GroupConnection{
		Nodes:      []*model.Group{},
		TotalCount: int64(len(groups)),
	}

	for _, group := range groups {
		grp := &model.Group{
			ID:          &group.ID,
			CreatedAt:   &group.CreatedAt,
			Name:        group.Name,
			Description: group.Description,
			AvatarURL:   nil,
		}
		ret.Nodes = append(ret.Nodes, grp)
	}
	return ret, nil
}
