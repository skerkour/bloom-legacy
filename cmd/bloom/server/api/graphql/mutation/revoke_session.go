package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
)

func (r *Resolver) RevokeSession(ctx context.Context, input model.RevokeSessionInput) (ret bool, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	err = users.RevokeSession(ctx, currentUser, input.ID)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = true
	return
}
