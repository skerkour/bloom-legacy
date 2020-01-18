package accounts

import (
	"context"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"time"
)

func VerifyPendingAccount(ctx context.Context, tx *sqlx.Tx, pendingAccount PendingAccount, code string) twirp.Error {
	if pendingAccount.Trials+1 >= 10 {
		return twirp.NewError(twirp.PermissionDenied, "Maximum trials reached. Please create a new account.")
	}

	if !argon2id.VerifyPassword(code, pendingAccount.VerificationCodeHash) {
		return twirp.InvalidArgumentError("code", "Verification code is not valid.")
	}

	now := time.Now().UTC()
	since := now.Sub(pendingAccount.UpdatedAt)
	if since >= 30*time.Minute {
		return twirp.InvalidArgumentError("code", "Verification code expired. Please create a new account.")
	}

	_, err := tx.Exec("UPDATE pending_accounts SET verified = TRUE, updated_at = $1 WHERE id = $2", now, pendingAccount.ID)
	if err != nil {
		return twirp.NewError(twirp.PermissionDenied, ErrorVerifyPendingAccountMsg)
	}
	return nil
}
