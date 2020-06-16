package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/gobox/uuid"
)

// EnableUser is used by instance's admin to re-enable a previously disabled user
func (resolver *Resolver) EnableUser(ctx context.Context, id uuid.UUID) (ret bool, err error) {
	err = resolver.usersService.EnableUser(ctx, id)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return
}
