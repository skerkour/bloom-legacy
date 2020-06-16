package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// Users finds all users
func (resolver *Resolver) Users(ctx context.Context) (ret *model.UserConnection, err error) {
	me, err := resolver.usersService.Me(ctx)
	if err != nil {
		err = api.NewError(err)
		return
	}

	users, err := resolver.usersService.FindAllUsers(ctx)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = &model.UserConnection{
		Nodes:      []*model.User{},
		TotalCount: int64(len(users)),
	}

	for _, user := range users {
		ret.Nodes = append(ret.Nodes, model.DomainUserToModelUser(me, user))
	}
	return
}
