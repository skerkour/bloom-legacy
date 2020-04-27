package mutation

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/rz"
)

func (r *Resolver) SignIn(ctx context.Context, input model.SignInInput) (ret *model.SignedIn, err error) {
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)
	apiCtx := apiutil.ApiCtxFromCtx(ctx)

	if apiCtx == nil {
		logger.Error("mutation.SignIn: error getting apiCtx from context")
		return ret, gqlerrors.Internal()
	}

	if currentUser != nil {
		return ret, gqlerrors.MustNotBeAuthenticated()
	}

	// sleep to prevent spam and bruteforce
	sleep, err := crypto.RandInt64(500, 800)
	if err != nil {
		logger.Error("mutation.SignIn: generating random int", rz.Err(err))
		err = gqlerrors.Internal()
		return
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	device := users.SessionDevice{
		OS:   input.Device.Os.String(),
		Type: input.Device.Type.String(),
	}
	params := users.SignInParams{
		Username: input.Username,
		AuthKey:  input.AuthKey,
		Device:   device,
	}
	user, newSession, token, err := users.SignIn(ctx, params)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	// TODO: send login email
	ret = &model.SignedIn{
		Session: &model.Session{
			ID:    newSession.ID,
			Token: &token,
			Device: &model.SessionDevice{
				Os:   model.SessionDeviceOs(device.OS),
				Type: model.SessionDeviceType(device.Type),
			},
		},
		Me: &model.User{
			ID:          &user.ID,
			AvatarURL:   nil,
			CreatedAt:   &user.CreatedAt,
			Username:    user.Username,
			FirstName:   &user.FirstName,
			LastName:    &user.LastName,
			DisplayName: user.DisplayName,
			IsAdmin:     user.IsAdmin,
		},
	}

	return
}
