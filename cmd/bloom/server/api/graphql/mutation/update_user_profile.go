package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/uuid"
)

func (r *Resolver) UpdateUserProfile(ctx context.Context, input model.UserProfileInput) (*model.User, error) {
	var ret *model.User
	var err error
	currentUser := apiutil.UserFromCtx(ctx)
	var userId uuid.UUID

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	if input.ID != nil {
		userId = uuid.MustParse(*input.ID)
	}
	params := users.UpdateProfileInput{
		ID:          &userId,
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
