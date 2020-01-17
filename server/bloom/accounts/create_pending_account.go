package accounts

import (
	"context"
	"fmt"
	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/core/bloom/accounts"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/libs/rz-go"
	"time"
)

func CreatePendingAccount(ctx context.Context, tx *sqlx.Tx, displayName, email string) (PendingAccount, string, twirp.Error) {
	logger := rz.FromCtx(ctx)
	var existingAccount int
	var err error

	// validate params
	if err = accounts.ValidateDisplayName(displayName); err != nil {
		return PendingAccount{}, "", twirp.InvalidArgumentError("display_name", err.Error())
	}

	if err = accounts.ValidateEmail(email, config.DisposableEmailDomains); err != nil {
		return PendingAccount{}, "", twirp.InvalidArgumentError("email", err.Error())
	}

	// check if email does not already exist
	queryCountExistingEmails := "SELECT COUNT(*) FROM accounts WHERE email = $1"
	err = tx.Get(&existingAccount, queryCountExistingEmails, email)
	if err != nil {
		logger.Error("accounts.CreatePendingAccount: error fetching existing emails counts", rz.Err(err))
		return PendingAccount{}, "", twirp.InternalError("error creating new account")
	}

	if existingAccount != 0 {
		return PendingAccount{}, "", twirp.InvalidArgumentError("email", fmt.Sprintf("account with email: '%s' already exists", email))
	}

	// TODO: generate verification code, hash verification code
	/*
			let verification_code = utils::random_digit_string(8);
		        // let auth_key_hash = argon2id::hash_password(
		        //     self.auth_key.as_bytes(),
		        //     accounts::PASSWORD_ARGON2_OPSLIMIT,
		        //     accounts::PASSWORD_ARGON2_MEMLIMIT,
		        // )?
		        // .to_string();
		        let verification_code_hash = argon2id::hash_password(
		            verification_code.as_bytes(),
		            accounts::PENDING_USER_CODE_ARGON2_OPSLIMIT,
		            accounts::PENDING_USER_CODE_ARGON2_MEMLIMIT,
		        )?
				.to_string();
	*/
	now := time.Now().UTC()
	verificationCode := "00000000"
	newUuid := uuid.New()
	ret := PendingAccount{
		ID:                   newUuid.String(),
		CreatedAt:            now,
		UpdatedAt:            now,
		Email:                email,
		DisplayName:          displayName,
		VerificationCodeHash: []byte(verificationCode),
		Trials:               0,
		Verified:             false,
	}

	queryCreatePendingAccount := `INSERT INTO pending_accounts
		(id, created_at, updated_at, email, display_name, verification_code_hash, trials, verified)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`
	_, err = tx.Exec(queryCreatePendingAccount, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Email, ret.DisplayName, ret.VerificationCodeHash, ret.Trials, ret.Verified)
	if err != nil {
		logger.Error("error creating new account", rz.Err(err))
		return ret, "", twirp.InternalError("error creating new account")
	}
	return ret, verificationCode, nil
}
