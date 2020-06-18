package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/crypto"
)

func (service *UsersService) SignIn(ctx context.Context, params users.SignInParams) (user users.User, session users.Session, token string, err error) {
	_, err = service.Me(ctx)
	if err == nil {
		err = users.ErrMustNotBeAuthenticated
		return
	}

	// sleep to prevent spam and bruteforce
	sleep, err := crypto.RandInt64(500, 800)
	if err != nil {
		sleep = 650
		err = nil
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	user, err = service.usersRepo.FindUserByUsername(ctx, service.db, params.Username)
	if err != nil {
		return
	}

	// verify password
	if !crypto.VerifyPasswordHash(params.AuthKey, user.AuthKeyHash) {
		err = users.ErrInvalidUsernamePasswordCombination
		return
	}

	session, token, err = newSession(ctx, user.ID, params.Device)
	if err != nil {
		return
	}
	err = service.usersRepo.CreateSession(ctx, service.db, session)
	if err != nil {
		return
	}

	go service.sendSignInAlertEmail(ctx, user, params.IPAddress)

	return
}
