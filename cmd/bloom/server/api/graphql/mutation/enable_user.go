package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/uuid"
)

func (r *Resolver) EnableUser(ctx context.Context, id uuid.UUID) (ret bool, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		err = gqlerrors.AuthenticationRequired()
		return
	}

	err = users.EnableUser(ctx, currentUser, id)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = true
	return
}
