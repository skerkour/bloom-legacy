package service

import (
	"context"
	"fmt"
	"strings"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *UsersService) CompleteRegistration(ctx context.Context, params users.CompleteRegistrationParams) (user users.User, session users.Session, token string, err error) {
	_, err = service.Me(ctx)
	if err == nil {
		err = users.ErrMustNotBeAuthenticated
		return
	}
	logger := log.FromCtx(ctx)
	zeroNonce := make([]byte, crypto.AEADNonceSize)

	// sleep to prevent spam and bruteforce
	sleep, err := crypto.RandInt64(500, 800)
	if err != nil {
		sleep = 650
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	// validate params
	params.Username = strings.TrimSpace(params.Username)
	err = validateUsername(params.Username)
	if err != nil {
		return
	}
	if len(params.PrivateKeyNonce) != crypto.AEADNonceSize {
		err = errors.InvalidArgument("privateKeyNonce has bad size")
		return
	}
	if crypto.ConstantTimeCompare(params.PrivateKeyNonce, zeroNonce) {
		err = errors.InvalidArgument("privateKeyNonce cannot be empty")
		return
	}
	if len(params.MasterKeyNonce) != crypto.AEADNonceSize {
		err = errors.InvalidArgument(fmt.Sprintf("masterKeyNonce has bad size (%d)", len(params.MasterKeyNonce)))
		return
	}
	if crypto.ConstantTimeCompare(params.MasterKeyNonce, zeroNonce) {
		err = errors.InvalidArgument("masterKeyNonce cannot be empty")
		return
	}

	authKeyHash, err := crypto.HashPassword(params.AuthKey, users.AuthKeyHashParams)
	if err != nil {
		errMessage := "users.CompleteRegistration: hashing auth key"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "users.CompleteRegistration: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	pendingUser, err := service.usersRepo.FindPendingUserByID(ctx, tx, params.PendingUserID)
	if err != nil {
		tx.Rollback()
		return
	}

	if pendingUser.VerifiedAt == nil {
		tx.Rollback()
		err = users.ErrPendingUserNotVerified
		return
	}

	// check if email is not already in use
	_, err = service.usersRepo.FindUserByEmail(ctx, tx, pendingUser.Email)
	if err != nil {
		if _, ok := err.(*errors.NotFoundError); !ok {
			tx.Rollback()
			return
		}
	}
	if err == nil {
		err = users.ErrEmailAlreadyInUse
		tx.Rollback()
		return
	}

	// verify that username isn't already in use
	_, err = service.usersRepo.FindUserByUsername(ctx, tx, params.Username)
	if err != nil {
		if _, ok := err.(*errors.NotFoundError); !ok {
			tx.Rollback()
			return
		}
	}
	if err == nil {
		err = users.ErrUsernameAlreadyInUse
		tx.Rollback()
		return
	}

	now := time.Now().UTC()
	user = users.User{
		ID:                  uuid.New(),
		CreatedAt:           now,
		UpdatedAt:           now,
		DisabledAt:          nil,
		Username:            params.Username,
		Email:               pendingUser.Email,
		DisplayName:         pendingUser.DisplayName,
		Bio:                 "",
		FirstName:           "",
		LastName:            "",
		State:               0,
		IsAdmin:             false,
		AuthKeyHash:         authKeyHash,
		PublicKey:           params.PublicKey,
		EncryptedPrivateKey: params.EncryptedPrivateKey,
		PrivateKeyNonce:     params.PrivateKeyNonce,
		EncryptedMasterKey:  params.EncryptedMasterKey,
		MasterKeyNonce:      params.MasterKeyNonce,
	}
	err = service.usersRepo.CreateUser(ctx, tx, user)
	if err != nil {
		tx.Rollback()
		return
	}

	session, token, err = newSession(ctx, user.ID, params.Device)
	if err != nil {
		tx.Rollback()
		return
	}
	err = service.usersRepo.CreateSession(ctx, tx, session)
	if err != nil {
		tx.Rollback()
		return
	}

	err = service.usersRepo.DeletePendingUser(ctx, tx, pendingUser.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	_, err = service.bilingService.CreateCustomer(ctx, tx, user, nil)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "users.CompleteRegistration: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
