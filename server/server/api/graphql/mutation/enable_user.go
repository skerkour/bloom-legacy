package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

// EnableUser is used by instance's admin to re-enable a previously disabled user
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
