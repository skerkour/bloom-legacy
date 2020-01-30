package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apictx"
	gqlerrors "gitlab.com/bloom42/bloom/server/api/graphql/errors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/libs/rz-go"
)

func (resolver *Resolver) Me(ctx context.Context) (*model.User, error) {
	var ret *model.User
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)

	if !ok {
		logger.Error("query.Me: error getting apiCtx from context")
		return ret, gqlerrors.Internal()
	}
	if apiCtx.AuthenticatedUser != nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	currentUser := apiCtx.AuthenticatedUser

	ret = &model.User{
		ID:          &currentUser.ID,
		CreatedAt:   &currentUser.CreatedAt,
		Username:    currentUser.Username,
		FirstName:   &currentUser.FirstName,
		LastName:    &currentUser.LastName,
		DisplayName: currentUser.DisplayName,
	}

	return ret, nil
}
