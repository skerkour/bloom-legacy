package users

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/errors"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

func SendNewRegistrationCode(ctx context.Context, pendingUserID uuid.UUID) (err error) {
	logger := rz.FromCtx(ctx)
	var pendingUser PendingUser

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.SendNewRegistrationCode: Starting transaction", rz.Err(err))
		err = NewError(ErrorVerifyingPendingUser)
		return
	}

	err = tx.Get(&pendingUser, "SELECT * FROM pending_users WHERE id = $1 FOR UPDATE", pendingUserID)
	if err != nil {
		tx.Rollback()
		logger.Error("users.SendNewRegistrationCode: getting pending user", rz.Err(err))
		err = NewError(ErrorVerifyingPendingUser)
		return
	}

	now := time.Now().UTC()
	since := now.Sub(pendingUser.UpdatedAt)
	if since <= 30*time.Second {
		tx.Rollback()
		err = errors.New(errors.PermissionDenied, "Please wait at least 30 seconds before requesting a new code.")
		return
	}

	// generate new code and update pending user
	verificationCode, err := generateNewRegistrationCode(ctx, tx, &pendingUser)
	if err != nil {
		tx.Rollback()
		return
	}

	err = sendUserVerificationCode(pendingUser.Email, pendingUser.DisplayName, verificationCode)
	if err != nil {
		tx.Rollback()
		logger.Error("users.SendNewRegistrationCode: sending email", rz.Err(err))
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutaiton.SendNewRegistrationCode: committing transaction", rz.Err(err))
		err = NewError(ErrorVerifyingPendingUser)
		return
	}

	return
}
