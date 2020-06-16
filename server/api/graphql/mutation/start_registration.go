package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// StartRegistration initiate an account registration
func (resolver *Resolver) StartRegistration(ctx context.Context, input model.StartRegistrationInput) (ret *model.RegistrationStarted, err error) {
	params := users.StartRegistrationParams{
		DisplayName: input.DisplayName,
		Email:       input.Email,
	}
	newPendingUserID, err := resolver.usersService.StartRegistration(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.RegistrationStarted{
		ID: newPendingUserID,
	}
	return
}
