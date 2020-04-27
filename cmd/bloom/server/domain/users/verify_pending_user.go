package users

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

// VerifyPendingUserParams are the parameters for VerifyPendingUser
type VerifyPendingUserParams struct {
	PendingUserID uuid.UUID
	Code          string
}

// VerifyPendingUser verifies a pending user
func VerifyPendingUser(ctx context.Context, params VerifyPendingUserParams) (err error) {
	logger := rz.FromCtx(ctx)
	var pendingUser PendingUser

	// verify pending user
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.VerifyPendingUser: Starting transaction", rz.Err(err))
		err = NewError(ErrorVerifyingPendingUser)
		return
	}

	err = tx.Get(&pendingUser, "SELECT * FROM pending_users WHERE id = $1 FOR UPDATE", params.PendingUserID)
	if err != nil {
		tx.Rollback()
		logger.Error("users.VerifyPendingUser: getting pending user", rz.Err(err),
			rz.String("pending_user.id", params.PendingUserID.String()))
		err = NewError(ErrorVerifyingPendingUser)
		return
	}

	if pendingUser.FailedAttempts+1 >= MAX_REGISTRATION_ATTEMPTS {
		tx.Rollback()
		err = NewError(ErrorMaximumVerificationTrialsReached)
		return
	}

	now := time.Now().UTC()
	since := now.Sub(pendingUser.UpdatedAt)
	if since >= 30*time.Minute {
		tx.Rollback()
		err = NewError(ErrorRegistrationCodeExpired)
		return
	}

	if !crypto.VerifyPasswordHash([]byte(params.Code), pendingUser.VerificationCodeHash) {
		tx.Rollback()
		err = NewError(ErrorRegistrationCodeIsNotValid)
		tx2, _ := db.DB.Beginx()
		if tx2 != nil {
			err2 := failPendingUserVerification(ctx, tx2, &pendingUser)
			if err2 != nil {
				tx2.Rollback()
				err = NewError(ErrorVerifyingPendingUser)
				return
			}
			tx2.Commit()
		}
		return
	}

	now = time.Now().UTC()
	pendingUser.VerifiedAt = &now
	pendingUser.UpdatedAt = now

	_, err = tx.Exec("UPDATE pending_users SET verified_at = $1, updated_at = $1 WHERE id = $2",
		now, pendingUser.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("users.VerifyPendingUser: error verifying pending user", rz.Err(err),
			rz.String("pending_user.id", pendingUser.ID.String()))
		return NewError(ErrorVerifyingPendingUser)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.VerifyPendingUser: Committing transaction", rz.Err(err))
		err = NewError(ErrorVerifyingPendingUser)
		return
	}

	return
}
