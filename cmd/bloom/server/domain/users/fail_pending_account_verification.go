package users

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/gobox/rz"
)

func failPendingUserVerification(ctx context.Context, tx *sqlx.Tx, pendingUser *PendingUser) error {
	logger := rz.FromCtx(ctx)

	now := time.Now().UTC()

	pendingUser.FailedAttempts += 1

	_, err := tx.Exec("UPDATE pending_users SET failed_verifications = $1, updated_at = $2 WHERE id = $3",
		pendingUser.FailedAttempts, now, pendingUser.ID)
	if err != nil {
		logger.Error("VerifyPendingUser: error verifying pending user", rz.Err(err),
			rz.String("pending_user.id", pendingUser.ID.String()))
		return NewError(ErrorVerifyingPendingUser)
	}

	return nil
}
