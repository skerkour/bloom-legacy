package users

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

type StartRegistrationParams struct {
	DisplayName string
	Email       string
}

func StartRegistration(ctx context.Context, params StartRegistrationParams) (newPendingUserID uuid.UUID, err error) {
	logger := rz.FromCtx(ctx)

	// create pending user
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.StartRegistration: Starting transaction", rz.Err(err))
		err = NewError(ErrorCreatingPendingUser)
		return
	}

	newPendingUser, verificationCode, err := createPendingUser(ctx, tx, params.DisplayName, params.Email)
	if err != nil {
		tx.Rollback()
		return
	}

	err = sendUserVerificationCode(newPendingUser.Email, newPendingUser.DisplayName, verificationCode)
	if err != nil {
		tx.Rollback()
		logger.Error("users.StartRegistration: Sending confirmation email", rz.Err(err))
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.StartRegistration: Committing transaction", rz.Err(err))
		err = NewError(ErrorCreatingPendingUser)
		return
	}

	newPendingUserID = newPendingUser.ID
	return
}
