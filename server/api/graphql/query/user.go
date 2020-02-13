package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// User finds an user
func (resolver *Resolver) User(ctx context.Context, username *string) (*model.User, error) {
	var ret *model.User
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AdminRoleRequired()
	}

	if username == nil {
		return ret, gqlerrors.Internal()
	}

	user, err := users.FindUserByUsernameNoTx(ctx, *username)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &model.User{
		ID:          &user.ID,
		AvatarURL:   nil,
		CreatedAt:   &user.CreatedAt,
		Username:    user.Username,
		FirstName:   &user.FirstName,
		LastName:    &user.LastName,
		DisplayName: user.DisplayName,
		IsAdmin:     user.IsAdmin,
		Bio:         user.Bio,
		Email:       &user.Email,
	}

	return ret, nil
}
