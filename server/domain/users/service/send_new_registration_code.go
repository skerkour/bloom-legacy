package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) SendNewRegistrationCode(ctx context.Context, pendingUserID uuid.UUID) (err error) {
	_, err = service.Me(ctx)
	if err == nil {
		err = users.ErrMustNotBeAuthenticated
		return
	}
	logger := log.FromCtx(ctx)

	pendingUser, err := service.usersRepo.FindPendingUserByID(ctx, service.db, pendingUserID)
	if err != nil {
		return
	}

	now := time.Now().UTC()
	since := now.Sub(pendingUser.UpdatedAt)
	if since <= 30*time.Second {
		err = errors.InvalidArgument("Please wait at least 30 seconds before requesting a new code")
		return
	}

	codeBytes, err := crypto.RandAlphabet([]byte(users.RegisterCodeAlphabet), users.RegisterCodeLength)
	if err != nil {
		errMessage := "users.SendNewRegistrationCode: generating code"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	codeHash, err := crypto.HashPassword(codeBytes, users.RegisterCodeHashParams)
	if err != nil {
		errMessage := "users.SendNewRegistrationCode: hashing code"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	pendingUser.CodeHash = codeHash
	pendingUser.UpdatedAt = time.Now().UTC()
	err = service.usersRepo.UpdatePendingUser(ctx, service.db, pendingUser)
	if err != nil {
		return
	}

	go service.sendRegisterEmail(ctx, formatCodeHyphen(string(codeBytes)), pendingUser)

	// sleep to prevent spam and bruteforce
	sleep, err := crypto.RandInt64(500, 800)
	if err != nil {
		sleep = 650
		err = nil
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)
	return
}
