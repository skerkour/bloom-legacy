package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
)

func (service *UsersService) SignIn(ctx context.Context, params users.SignInParams) (user users.User, session users.Session, token string, err error) {
	return
}

/*
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
		IPAddress: apiCtx.IP,
	}
	user, newSession, pendingSession, token, err := users.SignIn(ctx, params)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}


*/
