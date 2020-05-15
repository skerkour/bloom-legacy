package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

// CompleteRegistrationParams are the parameters for CompleteRegistration
type CompleteRegistrationParams struct {
	PendingUserID       uuid.UUID
	Username            string
	AuthKey             []byte
	Device              SessionDevice
	PublicKey           []byte
	EncryptedPrivateKey []byte
	PrivateKeyNonce     []byte
	EncryptedMasterKey  []byte
	MasterKeyNonce      []byte
}

// CompleteRegistration is used to complete the registration of an account and create an user
func CompleteRegistration(ctx context.Context, tx *sqlx.Tx, params CompleteRegistrationParams) (retUser *User, retSession *Session, token string, err error) {
	logger := rz.FromCtx(ctx)
	var pendingUser PendingUser

	// find pending user
	err = tx.Get(&pendingUser, "SELECT * FROM pending_users WHERE id = $1 FOR UPDATE", params.PendingUserID)
	if err != nil {
		logger.Error("users.CompleteRegistration: finding pending user", rz.Err(err))
		err = NewError(ErrorCompletingRegistration)
		return
	}

	// delete pending user
	err = deletePendingUser(ctx, tx, pendingUser.ID)
	if err != nil {
		return
	}

	// create user
	createUserParams := createUserParams{
		Email:               pendingUser.Email,
		DisplayName:         pendingUser.DisplayName,
		Username:            params.Username,
		AuthKey:             params.AuthKey,
		PublicKey:           params.PublicKey,
		EncryptedPrivateKey: params.EncryptedPrivateKey,
		PrivateKeyNonce:     params.PrivateKeyNonce,
		EncryptedMasterKey:  params.EncryptedMasterKey,
		MasterKeyNonce:      params.MasterKeyNonce,
	}
	retUser, err = createUser(ctx, tx, createUserParams)
	if err != nil {
		return
	}

	retSession, token, err = startSession(ctx, tx, retUser.ID, params.Device)
	if err != nil {
		return
	}

	globalSessionsCache.Set(retSession)

	return
}
