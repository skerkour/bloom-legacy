package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (r *Resolver) UpdateUserProfile(ctx context.Context, input model.UserProfileInput) (*model.User, error) {
	var ret *model.User
	var err error
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	params := users.UpdateProfileInput{
		ID:          input.ID,
		DisplayName: input.DisplayName,
		FirstName:   input.FirstName,
		LastName:    input.LastName,
		Bio:         input.Bio,
	}

	user, err := users.UpdateProfile(ctx, currentUser, params)
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
