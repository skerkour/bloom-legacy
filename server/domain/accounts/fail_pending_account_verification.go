package accounts

import (
	"context"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/rz-go"
	"time"
)

func FailPendingAccountVerification(ctx context.Context, tx *sqlx.Tx, pendingAccount PendingAccount) twirp.Error {
	logger := rz.FromCtx(ctx)

	now := time.Now().UTC()

	_, err := tx.Exec("UPDATE pending_accounts SET trials = $1, updated_at = $2 WHERE id = $3", pendingAccount.Trials+1, now, pendingAccount.ID)
	if err != nil {
		logger.Error("VerifyPendingAccount: error verifying pending account", rz.Err(err), rz.String("pending_account_id", pendingAccount.ID))
		return twirp.NewError(twirp.Internal, "internal error")
	}

	return nil
}
