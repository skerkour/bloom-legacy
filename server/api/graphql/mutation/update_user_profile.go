package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// UpdateUserProfile is used to updated an user's profile
func (resolver *Resolver) UpdateUserProfile(ctx context.Context, input model.UserProfileInput) (ret *model.User, err error) {
	me, err := resolver.usersService.Me(ctx)
	if err != nil {
		err = api.NewError(err)
		return
	}

	params := users.UpdateUserProfileParams{
		UserID:      input.ID,
		DisplayName: input.DisplayName,
		FirstName:   input.FirstName,
		LastName:    input.LastName,
		Bio:         input.Bio,
	}
	user, err := resolver.usersService.UpdateUserProfile(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = model.DomainUserToModelUser(me, user)
	return ret, nil
}
