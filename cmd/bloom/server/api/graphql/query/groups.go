package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
)

// Groups finds all groups
func (r *Resolver) Groups(ctx context.Context) (ret *model.GroupConnection, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		err = gqlerrors.AdminRoleRequired()
		return
	}

	groups, err := groups.FindAllGroups(ctx)
	if err != nil {
		err = gqlerrors.New(err)
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
