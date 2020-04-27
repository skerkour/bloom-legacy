package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
)

func (r *Resolver) DeleteGroup(ctx context.Context, input model.DeleteGroupInput) (ret bool, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	err = groups.DeleteGroup(ctx, currentUser, input.ID)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = true
	return ret, nil
}
