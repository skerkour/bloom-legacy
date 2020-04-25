package users

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/rz"
)

func VerifyPendingUser(ctx context.Context, tx *sqlx.Tx, pendingUser *PendingUser, code string) error {
	logger := rz.FromCtx(ctx)

	if pendingUser.FailedAttempts+1 >= 5 {
		return NewError(ErrorMaximumVerificationTrialsReached)
	}

	now := time.Now().UTC()
	since := now.Sub(pendingUser.UpdatedAt)
	if since >= 30*time.Minute {
		return NewError(ErrorRegistrationCodeExpired)
	}

	if !crypto.VerifyPasswordHash([]byte(code), pendingUser.VerificationCodeHash) {
		return NewError(ErrorRegistrationCodeIsNotValid)
	}

	now = time.Now().UTC()
	pendingUser.VerifiedAt = &now
	pendingUser.UpdatedAt = now

	_, err := tx.Exec("UPDATE pending_users SET verified_at = $1, updated_at = $1 WHERE id = $2",
		now, pendingUser.ID)
	if err != nil {
		logger.Error("users.VerifyPendingUser: error verifying pending user", rz.Err(err),
			rz.String("pending_user.id", pendingUser.ID.String()))
		return NewError(ErrorVerifyingPendingUser)
	}
	return nil
}
