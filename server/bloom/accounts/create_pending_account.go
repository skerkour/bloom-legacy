package accounts

import (
	"context"
	"fmt"
	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/bloom/server/services/util"
	"gitlab.com/bloom42/libs/crypto42-go/kdf/argon2id"
	"gitlab.com/bloom42/libs/rz-go"
	"time"
)

const ErrorCreatePendingAccountMsg = "Error creating new account. Please try again."

func CreatePendingAccount(ctx context.Context, tx *sqlx.Tx, displayName, email string) (PendingAccount, string, twirp.Error) {
	logger := rz.FromCtx(ctx)
	var existingAccount int
	var err error

	// validate params
	if err = validator.AccountDisplayName(displayName); err != nil {
		return PendingAccount{}, "", twirp.InvalidArgumentError("display_name", err.Error())
	}

	if err = validator.AccountEmail(email, config.DisposableEmailDomains); err != nil {
		return PendingAccount{}, "", twirp.InvalidArgumentError("email", err.Error())
	}

	// check if email does not already exist
	queryCountExistingEmails := "SELECT COUNT(*) FROM accounts WHERE email = $1"
	err = tx.Get(&existingAccount, queryCountExistingEmails, email)
	if err != nil {
		logger.Error("accounts.CreatePendingAccount: error fetching existing emails counts", rz.Err(err))
		return PendingAccount{}, "", twirp.InternalError(ErrorCreatePendingAccountMsg)
	}

	if existingAccount != 0 {
		return PendingAccount{}, "", twirp.InvalidArgumentError("email", fmt.Sprintf("account with email: '%s' already exists", email))
	}

	now := time.Now().UTC()
	newUuid := uuid.New()
	verificationCode, err := util.RandomDigitStr(8)
	if err != nil {
		logger.Error("accounts.CreatePendingAccount: error generating verification code", rz.Err(err))
		return PendingAccount{}, "", twirp.InternalError(ErrorCreatePendingAccountMsg)
	}

	// TODO: update params
	codeHash, err := argon2id.HashPassword(verificationCode, argon2id.DefaultHashPasswordParams)
	ret := PendingAccount{
		ID:                   newUuid.String(),
		CreatedAt:            now,
		UpdatedAt:            now,
		Email:                email,
		DisplayName:          displayName,
		VerificationCodeHash: codeHash,
		Trials:               0,
		Verified:             false,
	}

	queryCreatePendingAccount := `INSERT INTO pending_accounts
		(id, created_at, updated_at, email, display_name, verification_code_hash, trials, verified)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`
	_, err = tx.Exec(queryCreatePendingAccount, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Email, ret.DisplayName, ret.VerificationCodeHash, ret.Trials, ret.Verified)
	if err != nil {
		logger.Error("error creating new account", rz.Err(err))
		return ret, "", twirp.InternalError(ErrorCreatePendingAccountMsg)
	}
	return ret, verificationCode, nil
}
