package users

import (
	"context"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/rz-go"
	"time"
)

func FailPendingUserVerification(ctx context.Context, tx *sqlx.Tx, pendingUser PendingUser) twirp.Error {
	logger := rz.FromCtx(ctx)

	now := time.Now().UTC()

	_, err := tx.Exec("UPDATE pending_users SET trials = $1, updated_at = $2 WHERE id = $3", pendingUser.Trials+1, now, pendingUser.ID)
	if err != nil {
		logger.Error("VerifyPendingUser: error verifying pending user", rz.Err(err), rz.String("pending_user_id", pendingUser.ID))
		return twirp.NewError(twirp.Internal, "internal error")
	}

	return nil
}
