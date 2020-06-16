package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// RevokeSession is used to revoke a session and / or sign out
func (resolver *Resolver) RevokeSession(ctx context.Context, input model.RevokeSessionInput) (ret bool, err error) {
	err = resolver.usersService.RevokeSession(ctx, input.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return
}
