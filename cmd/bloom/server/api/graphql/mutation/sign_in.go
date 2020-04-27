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

// SignIn is used to sign-in, or initiate a pending session if 2fa is enabled
func (r *Resolver) SignIn(ctx context.Context, input model.SignInInput) (ret *model.SignedIn, err error) {
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)
	apiCtx := apiutil.ApiCtxFromCtx(ctx)
	var retSession *model.Session
	var retPendingSession *model.PendingSession

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

	params := users.SignInParams{
		Username: input.Username,
		AuthKey:  input.AuthKey,
		Device: users.SessionDevice{
			OS:   input.Device.Os.String(),
			Type: input.Device.Type.String(),
		},
	}
	user, newSession, pendingSession, token, err := users.SignIn(ctx, params)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	if newSession != nil {
		retSession = &model.Session{
			ID:    newSession.ID,
			Token: &token,
			Device: &model.SessionDevice{
				Os:   model.SessionDeviceOs(newSession.DeviceOS),
				Type: model.SessionDeviceType(newSession.DeviceType),
			},
		}
	} else if pendingSession != nil {
		retPendingSession = &model.PendingSession{
			ID:    pendingSession.ID,
			Token: token,
			TwoFa: &model.TwoFa{
				Method: model.TwoFAMethodTotp,
			},
		}
	} else {
		err = gqlerrors.Internal()
		return
	}

	ret = &model.SignedIn{
		Session:        retSession,
		PendingSession: retPendingSession,
		Me:             model.DomainUserToModelUser(user, user),
	}
	return
}
