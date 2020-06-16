package users

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/db"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/rz"
)

// SignInParams are the parameters for SignIn
type SignInParams struct {
	Username  string
	AuthKey   []byte
	Device    SessionDevice
	IPAddress string
}

// SignIn is used to sign-in an user
func SignIn(ctx context.Context, params SignInParams) (user *User, newSession *Session, pendingSession *PendingSession, token string, err error) {
	logger := rz.FromCtx(ctx)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.SignIn: Starting transaction", rz.Err(err))
		err = NewError(ErrorSingingIn)
		return
	}

	// fetch user
	user, err = FindUserByUsername(ctx, tx, params.Username)
	if err != nil {
		tx.Rollback()
		err = NewError(ErrorUserNotFound)
		return
	}

	// verify password
	if !crypto.VerifyPasswordHash(params.AuthKey, user.AuthKeyHash) {
		tx.Rollback()
		err = NewError(ErrorInvalidUsernamePasswordCombination)
		return
	}

	newSession, token, err = startSession(ctx, tx, user.ID, params.Device)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.SignIn: committing transaction", rz.Err(err))
		return
	}

	// send alert email
	go sendSignInEmailAlert(user.Email, user.DisplayName, params.IPAddress)

	globalSessionsCache.Set(newSession)

	return
}
