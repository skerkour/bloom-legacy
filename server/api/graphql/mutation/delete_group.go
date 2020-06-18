package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// DeleteGroup is used by a group's admin to delete the group
func (resolver *Resolver) DeleteGroup(ctx context.Context, input model.DeleteGroupInput) (ret bool, err error) {
	err = resolver.groupsService.DeleteGroup(ctx, input.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return
}
