package accounts

import (
	"context"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/rz-go"
	"time"
)

func CreatePendingAccount(ctx context.Context, tx *sqlx.Tx, displayName, email string) (PendingAccount, twirp.Error) {
	logger := rz.FromCtx(ctx)
	// todo: valdiate displayName, email
	// check if emails does not already exists
	// generate ID, verification code, hash verification code

	now := time.Now().UTC()
	code := "000000"
	ret := PendingAccount{
		ID:                   "230690b1-265c-41b3-94f1-28a6c271dd98",
		CreatedAt:            now,
		UpdatedAt:            now,
		Email:                email,
		DisplayName:          displayName,
		VerificationCodeHash: []byte(code),
		Trials:               0,
		Verified:             false,
	}

	query := `INSERT INTO pending_accounts
		(id, created_at, updated_at, email, display_name, verification_code_hash, trials, verified)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`
	_, err := tx.Exec(query, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Email, ret.DisplayName, ret.VerificationCodeHash, ret.Trials, ret.Verified)
	if err != nil {
		logger.Error("error creating new account", rz.Err(err))
		return ret, twirp.InternalError("error creating new account")
	}
	return ret, nil
}
