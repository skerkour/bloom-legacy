package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/server/domain/groups"
)

// QuitGroup quit a group
func (r *Resolver) QuitGroup(ctx context.Context, input model.QuitGroupInput) (ret bool, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	err = groups.QuitGroup(ctx, currentUser, input.GroupID)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = true
	return
}
