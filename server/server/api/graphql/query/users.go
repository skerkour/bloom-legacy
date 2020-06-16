package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
)

// Users finds all users
func (resolver *Resolver) Users(ctx context.Context) (ret *model.UserConnection, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		err = gqlerrors.AdminRoleRequired()
		return
	}

	users, err := users.FindAllUsers(ctx)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = &model.UserConnection{
		Nodes:      []*model.User{},
		TotalCount: int64(len(users)),
	}

	for _, user := range users {
		ret.Nodes = append(ret.Nodes, model.DomainUserToModelUser(currentUser, &user))
	}
	return
}
