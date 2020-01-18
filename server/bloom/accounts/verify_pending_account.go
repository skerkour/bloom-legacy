package accounts

import (
	"context"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
)

func VerifyPendingAccount(ctx context.Context, tx *sqlx.Tx, pendingAccountId, code string) twirp.Error {
	return nil
}
