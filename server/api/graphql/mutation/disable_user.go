package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/gobox/uuid"
)

// DisableUser is used by instance's admins to disable an user
func (resolver *Resolver) DisableUser(ctx context.Context, id uuid.UUID) (ret bool, err error) {
	err = resolver.usersService.DisableUser(ctx, id)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return
}
