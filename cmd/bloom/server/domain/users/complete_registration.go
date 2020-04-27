package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

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

func CompleteRegistration(ctx context.Context, tx *sqlx.Tx, params CompleteRegistrationParams) (retUser *User, retSession *Session, token string, err error) {
	logger := rz.FromCtx(ctx)
	var pendingUser PendingUser

	// find pending user
	err = tx.Get(&pendingUser, "SELECT * FROM pending_users WHERE id = $1 FOR UPDATE", params.PendingUserID)
	if err != nil {
		tx.Rollback()
		logger.Error("users.CompleteRegistration: finding pending user", rz.Err(err))
		err = NewError(ErrorCompletingRegistration)
		return
	}

	// delete pending user
	err = DeletePendingUser(ctx, tx, pendingUser.ID.String())
	if err != nil {
		tx.Rollback()
		return
	}

	// create user
	createUserParams := createUserParams{
		PendingUser:         pendingUser,
		Username:            params.Username,
		AuthKey:             params.AuthKey,
		PublicKey:           params.PublicKey,
		EncryptedPrivateKey: params.EncryptedPrivateKey,
		PrivateKeyNonce:     params.PrivateKeyNonce,
	}
	retUser, err = createUser(ctx, tx, createUserParams)
	if err != nil {
		tx.Rollback()
		return
	}

	retSession, token, err = startSession(ctx, tx, retUser.ID, params.Device)
	if err != nil {
		tx.Rollback()
		return
	}

	GlobalSessionsCache.Set(retSession)

	return
}
