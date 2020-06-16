package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
)

// User finds an user
// username is optional for a future use
func (resolver *Resolver) User(ctx context.Context, username *string) (ret *model.User, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		err = gqlerrors.AuthenticationRequired()
		return
	}

	if username == nil {
		err = gqlerrors.Internal()
		return
	}

	user, err := users.FindUserByUsername(ctx, nil, *username)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = model.DomainUserToModelUser(currentUser, user)
	return
}
