package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

// DisableUser is used by instance's admins to disable an user
func (r *Resolver) DisableUser(ctx context.Context, id uuid.UUID) (ret bool, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		err = gqlerrors.AuthenticationRequired()
		return
	}

	err = users.DisableUser(ctx, currentUser, id)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = true
	return
}
