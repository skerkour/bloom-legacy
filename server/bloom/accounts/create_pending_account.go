package accounts

import (
	"context"
	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/core/bloom/accounts"
	"gitlab.com/bloom42/libs/rz-go"
	"time"
)

func CreatePendingAccount(ctx context.Context, tx *sqlx.Tx, displayName, email string) (PendingAccount, twirp.Error) {
	logger := rz.FromCtx(ctx)
	var existingAccounts int
	var err error

	if err = accounts.ValidateDisplayName(displayName); err != nil {
		return PendingAccount{}, twirp.InvalidArgumentError("display_name", err.Error())
	}

	// TODO: pass good data
	if err = accounts.ValidateEmail(email, map[string]bool{}); err != nil {
		return PendingAccount{}, twirp.InvalidArgumentError("email", err.Error())
	}

	// check if emails does not already exists
	queryCountExistingEmails := "SELECT COUNT(*) FROM accounts WHERE email = $1"
	err = tx.Get(&existingAccounts, queryCountExistingEmails, email)
	if err != nil {
		logger.Error("accounts.CreatePendingAccount: error fetching existing emails counts", rz.Err(err))
		return PendingAccount{}, twirp.InternalError("error creating new account")
	}

	// generate ID, verification code, hash verification code
	now := time.Now().UTC()
	code := "000000"
	newUuid := uuid.New()
	ret := PendingAccount{
		ID:                   newUuid.String(),
		CreatedAt:            now,
		UpdatedAt:            now,
		Email:                email,
		DisplayName:          displayName,
		VerificationCodeHash: []byte(code),
		Trials:               0,
		Verified:             false,
	}

	queryCreatePendingAccount := `INSERT INTO pending_accounts
		(id, created_at, updated_at, email, display_name, verification_code_hash, trials, verified)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`
	_, err = tx.Exec(queryCreatePendingAccount, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Email, ret.DisplayName, ret.VerificationCodeHash, ret.Trials, ret.Verified)
	if err != nil {
		logger.Error("error creating new account", rz.Err(err))
		return ret, twirp.InternalError("error creating new account")
	}
	return ret, nil
}
