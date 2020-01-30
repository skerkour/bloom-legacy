package users

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/libs/rz-go"
)

func FailPendingUserVerification(ctx context.Context, tx *sqlx.Tx, pendingUser PendingUser) error {
	logger := rz.FromCtx(ctx)

	now := time.Now().UTC()

	_, err := tx.Exec("UPDATE pending_users SET failed_verifications = $1, updated_at = $2 WHERE id = $3", pendingUser.FailedVerifications+1, now, pendingUser.ID)
	if err != nil {
		logger.Error("VerifyPendingUser: error verifying pending user", rz.Err(err), rz.String("pending_user_id", pendingUser.ID))
		return NewError(ErrorVerifyingPendingUser)
	}

	return nil
}
