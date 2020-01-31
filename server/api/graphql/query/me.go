package query

import (
	"context"

	gqlerrors "gitlab.com/bloom42/bloom/server/api/graphql/errors"
	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

func (resolver *Resolver) Me(ctx context.Context) (*model.User, error) {
	var ret *model.User
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	ret = &model.User{
		ID:          &currentUser.ID,
		CreatedAt:   &currentUser.CreatedAt,
		Username:    currentUser.Username,
		FirstName:   &currentUser.FirstName,
		LastName:    &currentUser.LastName,
		DisplayName: currentUser.DisplayName,
		IsAdmin:     currentUser.IsAdmin,
	}

	return ret, nil
}
