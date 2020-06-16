package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// QuitGroup quit a group
func (resolver *Resolver) QuitGroup(ctx context.Context, input model.QuitGroupInput) (ret bool, err error) {
	err = resolver.groupsService.QuitGroup(ctx, input.GroupID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return
}
