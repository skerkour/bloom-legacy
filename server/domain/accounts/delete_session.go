package accounts

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeleteSession(ctx context.Context, tx *sqlx.Tx, sessionId, accountId string) twirp.Error {
	logger := rz.FromCtx(ctx)

	queryDeleteSession := "DELETE FROM session WHERE id = $1 AND account_id = $2"
	_, err := tx.Exec(queryDeleteSession, sessionId, accountId)
	if err != nil {
		logger.Error("accounts.DeleteSession: error deleting sessiong", rz.Err(err),
			rz.String("session_id", sessionId), rz.String("account_id", accountId))
		return twirp.InternalError(ErrorDeleteSessionMsg)
	}
	return nil
}
