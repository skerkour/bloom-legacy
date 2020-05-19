package mutation

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/rz"
)

// StartRegistration initiate an account registration
func (resolver *Resolver) StartRegistration(ctx context.Context, input model.StartRegistrationInput) (ret *model.RegistrationStarted, err error) {
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser != nil {
		return ret, gqlerrors.MustNotBeAuthenticated()
	}

	// sleep to prevent spam and bruteforce
	sleep, err := crypto.RandInt64(1000, 1500)
	if err != nil {
		logger.Error("mutation.StartRegistration: generating random int", rz.Err(err))
		err = gqlerrors.Internal()
		return
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	params := users.StartRegistrationParams{
		DisplayName: input.DisplayName,
		Email:       input.Email,
	}
	newPendingUserID, err := users.StartRegistration(ctx, params)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = &model.RegistrationStarted{
		ID: newPendingUserID,
	}
	return
}
