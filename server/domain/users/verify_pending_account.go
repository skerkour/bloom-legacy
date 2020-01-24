package users

import (
	"context"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/rz-go"
	"time"
)

func VerifyPendingUser(ctx context.Context, tx *sqlx.Tx, pendingUser PendingUser, code string) twirp.Error {
	logger := rz.FromCtx(ctx)

	if pendingUser.FailedVerifications+1 >= 10 {
		return twirp.NewError(twirp.PermissionDenied, "Maximum trials reached. Please create a new account.")
	}

	if !argon2id.VerifyPassword([]byte(code), pendingUser.VerificationCodeHash) {
		return twirp.NewError(twirp.PermissionDenied, "Verification code is not valid.")
	}

	now := time.Now().UTC()
	since := now.Sub(pendingUser.UpdatedAt)
	if since >= 30*time.Minute {
		return twirp.InvalidArgumentError("code", "Verification code expired. Please create a new account.")
	}

	_, err := tx.Exec("UPDATE pending_users SET verified = TRUE, updated_at = $1 WHERE id = $2", now, pendingUser.ID)
	if err != nil {
		logger.Error("VerifyPendingUser: error verifying pending user", rz.Err(err), rz.String("pending_user_id", pendingUser.ID))
		return twirp.NewError(twirp.Internal, ErrorVerifyPendingUserMsg)
	}
	return nil
}
