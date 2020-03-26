package users

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/rz-go"
)

func VerifyPendingUser(ctx context.Context, tx *sqlx.Tx, pendingUser *PendingUser, code string) error {
	logger := rz.FromCtx(ctx)

	if pendingUser.FailedVerifications+1 >= 10 {
		return NewError(ErrorMaximumVerificationTrialsReached)
	}

	if !argon2id.VerifyPassword([]byte(code), pendingUser.VerificationCodeHash) {
		return NewError(ErrorRegistrationCodeIsNotValid)
	}

	now := time.Now().UTC()
	since := now.Sub(pendingUser.UpdatedAt)
	if since >= 30*time.Minute {
		return NewError(ErrorRegistrationCodeExpired)
	}

	_, err := tx.Exec("UPDATE pending_users SET verified = TRUE, updated_at = $1 WHERE id = $2",
		now, pendingUser.ID)
	if err != nil {
		logger.Error("VerifyPendingUser: error verifying pending user", rz.Err(err), rz.String("pending_user_id", pendingUser.ID))
		return NewError(ErrorVerifyingPendingUser)
	}
	return nil
}
