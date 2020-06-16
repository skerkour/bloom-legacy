package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/errors"
)

// User finds an user
// username is optional for a future use
func (resolver *Resolver) User(ctx context.Context, username *string) (ret *model.User, err error) {
	if username == nil {
		err = api.NewError(errors.Internal("query.User: username is nil", nil))
		return
	}

	user, err := resolver.usersService.FindUserByUsername(ctx, *username)
	if err != nil {
		err = api.NewError(err)
		return
	}

	me, _ := resolver.usersService.Me(ctx)

	ret = model.DomainUserToModelUser(me, user)
	return
}
