package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
)

func (r *Resolver) DisableUser(ctx context.Context, id string) (bool, error) {
	ret := false
	// logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}
	if !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	return ret, nil
}
