package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// Me returns the current user
func (resolver *Resolver) Me(ctx context.Context) (ret *model.User, err error) {
	user, err := resolver.usersService.Me(ctx)
	if err != nil {
		err = api.NewError(err)
		return
	}

	modeluser := model.DomainUserToModelUser(currentUser, currentUser)
	ret = &modeluser
	return
}
