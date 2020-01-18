package accounts

import (
	"context"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/rz-go"
)

func CreateAccount(ctx context.Context, tx *sqlx.Tx, pendingAccount PendingAccount, username string, authKey []byte) (Account, twirp.Error) {
	logger := rz.FromCtx(ctx)
	var err error
	var ret Account
	var existingAccount int

	// validate params
	if err = validator.AccountUsername(username); err != nil {
		return ret, twirp.InvalidArgumentError("username", err.Error())
	}

	// check if email does not already exist
	queryCountExistingEmails := "SELECT COUNT(*) FROM accounts WHERE email = $1"
	err = tx.Get(&existingAccount, queryCountExistingEmails, pendingAccount.Email)
	if err != nil {
		logger.Error("accounts.CreateAccount: error fetching existing emails counts", rz.Err(err))
		return ret, twirp.InternalError(ErrorCompletingRegistrationMsg)
	}
	if existingAccount != 0 {
		return ret, twirp.InvalidArgumentError("email", fmt.Sprintf("account with email: '%s' already exists", pendingAccount.Email))
	}

	// verify that username isn't already in use
	existingAccount = 0
	queryCountExistingUsername := "SELECT COUNT(*) FROM accounts WHERE username = $1"
	err = tx.Get(&existingAccount, queryCountExistingUsername, username)
	if err != nil {
		logger.Error("accounts.CreateAccount: error fetching existing username counts", rz.Err(err))
		return ret, twirp.InternalError(ErrorCompletingRegistrationMsg)
	}
	if existingAccount != 0 {
		return ret, twirp.InvalidArgumentError("email", fmt.Sprintf("Username '%s' is already is use", username))
	}

	// verify that username was not used by a deleted account
	existingAccount = 0
	queryCountDeletedUsername := "SELECT COUNT(*) FROM deleted_usernames WHERE username = $1"
	err = tx.Get(&existingAccount, queryCountDeletedUsername, username)
	if err != nil {
		logger.Error("accounts.CreateAccount: error fetching deleted username counts", rz.Err(err))
		return ret, twirp.InternalError(ErrorCompletingRegistrationMsg)
	}
	if existingAccount != 0 {
		return ret, twirp.InvalidArgumentError("email", fmt.Sprintf("Username '%s' is already is use", username))
	}

	now := time.Now().UTC()
	newUuid := uuid.New()
	// TODO: update params
	authKeyHash, err := argon2id.HashPassword(authKey, argon2id.DefaultHashPasswordParams)
	if err != nil {
		logger.Error("accounts.CreateAccount: hashing auth key", rz.Err(err))
		return ret, twirp.InternalError(ErrorCompletingRegistrationMsg)
	}

	ret = Account{
		ID:          newUuid.String(),
		Username:    username,
		Email:       pendingAccount.Email,
		CreatedAt:   now,
		UpdatedAt:   now,
		DisplayName: pendingAccount.DisplayName,
		AuthKeyHash: authKeyHash,
	}

	queryCreateAccount := `INSERT INTO accounts
		(id, created_at, updated_at, username, display_name, bio, email, first_name, last_name, is_admin, auth_key_hash)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)`
	_, err = tx.Exec(queryCreateAccount, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Username, ret.DisplayName, "", ret.Email, "", "", false, ret.AuthKeyHash)
	if err != nil {
		logger.Error("accounts.CreateAccount: inserting new account", rz.Err(err))
		return ret, twirp.InternalError(ErrorCompletingRegistrationMsg)
	}

	return ret, nil
}
