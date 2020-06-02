package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
)

// UpdateUserProfile is used to updated an user's profile
func (r *Resolver) UpdateUserProfile(ctx context.Context, input model.UserProfileInput) (*model.User, error) {
	var ret *model.User
	var err error
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	params := users.UpdateProfileParams{
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

	ret = model.DomainUserToModelUser(currentUser, user)
	return ret, nil
}
