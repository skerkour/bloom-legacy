package service

import (
	"context"
	"strings"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) StartRegistration(ctx context.Context, params users.StartRegistrationParams) (newPendingUserID uuid.UUID, err error) {
	_, err = service.Me(ctx)
	if err == nil {
		err = users.ErrMustNotBeAuthenticated
		return
	}
	logger := log.FromCtx(ctx)

	// clean and validate params
	params.DisplayName = strings.TrimSpace(params.DisplayName)
	err = validateDisplayName(params.DisplayName)
	if err != nil {
		return
	}

	params.Email = strings.ToLower(params.Email)
	params.Email = strings.TrimSpace(params.Email)
	err = validateEmail(params.Email, map[string]bool{})
	if err != nil {
		return
	}

	// check if email is not already in use
	_, err = service.usersRepo.FindUserByEmail(ctx, service.db, params.Email)
	if err != nil {
		if _, ok := err.(*errors.NotFoundError); !ok {
			return
		}
	}
	if err == nil {
		err = users.ErrEmailAlreadyInUse
		return
	}

	codeBytes, err := crypto.RandAlphabet([]byte(users.RegisterCodeAlphabet), users.RegisterCodeLength)
	if err != nil {
		errMessage := "users.StartRegistration: generating code"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	codeHash, err := crypto.HashPassword(codeBytes, users.RegisterCodeHashParams)
	if err != nil {
		errMessage := "users.StartRegistration: hashing code"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	now := time.Now().UTC()
	pendingUser := users.PendingUser{
		ID:             uuid.New(),
		CreatedAt:      now,
		UpdatedAt:      now,
		Email:          params.Email,
		DisplayName:    params.DisplayName,
		CodeHash:       codeHash,
		FailedAttempts: 0,
		VerifiedAt:     nil,
	}
	err = service.usersRepo.CreatePendingUser(ctx, service.db, pendingUser)
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
	newPendingUserID = pendingUser.ID
	return
}
