package service

import (
	"context"
	"strings"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/log"
)

func (service *UsersService) VerifyPendingUser(ctx context.Context, params users.VerifyPendingUserParams) (err error) {
	_, err = service.Me(ctx)
	if err == nil {
		err = users.ErrMustNotBeAuthenticated
		return
	}
	logger := log.FromCtx(ctx)

	// sleep to prevent spam and bruteforce
	sleep, err := crypto.RandInt64(500, 800)
	if err != nil {
		sleep = 650
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	// clean and validate data
	params.Code = strings.ToLower(params.Code)
	params.Code = strings.TrimSpace(params.Code)
	params.Code = cleanCodeHyphen(params.Code)

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "users.VerifyPendingUser: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	pendingUser, err := service.usersRepo.FindPendingUserByID(ctx, tx, params.PendingUserID)
	if err != nil {
		tx.Rollback()
		return
	}

	if pendingUser.FailedAttempts+1 >= users.RegistrationMaxFailedAttempts {
		tx.Rollback()
		err = users.ErrMaximumAttemptsReached
		return
	}

	now := time.Now().UTC()
	since := now.Sub(pendingUser.UpdatedAt)
	if since >= 30*time.Minute {
		tx.Rollback()
		err = users.ErrCodeExpired
		return
	}

	if !crypto.VerifyPasswordHash([]byte(params.Code), pendingUser.CodeHash) {
		tx.Rollback()
		err = users.ErrInvalidCode
		pendingUser.FailedAttempts += 1
		_ = service.usersRepo.UpdatePendingUser(ctx, service.db, pendingUser)
		return
	}

	now = time.Now().UTC()
	pendingUser.VerifiedAt = &now
	pendingUser.UpdatedAt = now
	err = service.usersRepo.UpdatePendingUser(ctx, tx, pendingUser)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "users.VerifyPendingUser: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
