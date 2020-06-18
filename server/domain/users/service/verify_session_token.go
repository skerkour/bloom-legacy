package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
)

func (service *UsersService) VerifySessionToken(ctx context.Context, token string) (user users.User, session users.Session, err error) {
	sessionID, sessionSecret, err := decodeSessionToken(ctx, token)
	if err != nil {
		return
	}

	session, err = service.userRepo.FindSessionByID(ctx, service.db, sessionID)
	if err != nil {
		if _, ok := err.(*errors.NotFoundError); ok {
			err = users.ErrInvalidSession
			return
		}
		return
	}

	err = verifySessionSecret(ctx, retSession, sessionSecret)
	if err != nil {
		return
	}

	user, err = service.userRepo.FindUserByID(ctx, service.db, retSession.UserID)
	if err != nil {
		if _, ok := err.(*errors.NotFoundError); ok {
			err = users.ErrInvalidSession
			return
		}
		return
	}
	return
}
