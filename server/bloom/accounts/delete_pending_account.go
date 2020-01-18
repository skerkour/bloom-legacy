package accounts

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeletePendingAccount(ctx context.Context, tx *sqlx.Tx, pendingAccountId string) twirp.Error {
	logger := rz.FromCtx(ctx)

	queryDeletePendingAccount := "DELETE FROM pending_accounts WHERE id = $1"
	_, err := tx.Exec(queryDeletePendingAccount, pendingAccountId)
	if err != nil {
		logger.Error("error deleting pending account", rz.Err(err), rz.String("pending_account_id", pendingAccountId))
		return twirp.InternalError(ErrorCompletingRegistrationMsg)
	}
	return nil
}
