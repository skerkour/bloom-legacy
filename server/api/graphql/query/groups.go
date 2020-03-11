package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

// Groups finds all groups
func (r *Resolver) Groups(ctx context.Context) (*model.GroupConnection, error) {
	var ret *model.GroupConnection
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	groups, err := groups.FindAllGroups(ctx)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &model.GroupConnection{
		Edges:      []*model.GroupEdge{},
		TotalCount: model.Int64(len(groups)),
	}

	for _, group := range groups {
		groupModel := &model.Group{
			ID:          &group.ID,
			CreatedAt:   &group.CreatedAt,
			Name:        group.Name,
			Description: group.Description,
			AvatarURL:   nil,
		}
		edge := &model.GroupEdge{
			Node: groupModel,
		}
		ret.Edges = append(ret.Edges, edge)
	}

	return ret, nil
}
