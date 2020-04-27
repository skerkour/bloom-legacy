package users

import (
	"context"
	"strings"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/config"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

func createPendingUser(ctx context.Context, tx *sqlx.Tx, displayName, email string) (ret *PendingUser, code string, err error) {
	logger := rz.FromCtx(ctx)
	var existingUser int

	// clean and validate params
	if err = validator.UserDisplayName(displayName); err != nil {
		err = NewErrorMessage(ErrorInvalidArgument, err.Error())
		return
	}

	email = strings.ToLower(email)
	email = strings.TrimSpace(email)
	if err = validator.UserEmail(email, config.DisposableEmailDomains); err != nil {
		err = NewErrorMessage(ErrorInvalidArgument, err.Error()) // twirp.InvalidArgumentError("email", err.Error())
		return
	}

	// check if email does not already exist
	queryCountExistingEmails := "SELECT COUNT(*) FROM users WHERE email = $1"
	err = tx.Get(&existingUser, queryCountExistingEmails, email)
	if err != nil {
		logger.Error("users.CreatePendingUser: error fetching existing emails counts", rz.Err(err))
		err = NewError(ErrorCreatingPendingUser)
		return
	}

	if existingUser != 0 {
		err = NewError(ErrorEmailAlreadyExists) // twirp.InvalidArgumentError("email", fmt.Sprintf("user with email: '%s' already exists", email))
		return
	}

	now := time.Now().UTC()
	newUuid := uuid.New()
	verificationCode, err := crypto.RandAlphabet([]byte(USER_VERIFICATION_CODE_ALPHABET), 8)
	if err != nil {
		logger.Error("users.CreatePendingUser: error generating verification code", rz.Err(err))
		err = NewError(ErrorCreatingPendingUser)
		return
	}
	code = string(verificationCode)

	// TODO: update params
	codeHash, err := crypto.HashPassword(verificationCode, PENDING_USER_CODE_HASH_PARAMS)
	if err != nil {
		logger.Error("users.CreatePendingUser: hashing verification code", rz.Err(err))
		err = NewError(ErrorCreatingPendingUser)
		return
	}

	ret = &PendingUser{
		ID:                   newUuid,
		CreatedAt:            now,
		UpdatedAt:            now,
		Email:                email,
		DisplayName:          displayName,
		VerificationCodeHash: codeHash,
		FailedAttempts:       0,
		VerifiedAt:           nil,
	}

	queryCreatePendingUser := `INSERT INTO pending_users
		(id, created_at, updated_at, email, display_name, verification_code_hash, failed_attempts, verified_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`
	_, err = tx.Exec(queryCreatePendingUser, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Email,
		ret.DisplayName, ret.VerificationCodeHash, ret.FailedAttempts, ret.VerifiedAt)
	if err != nil {
		logger.Error("error creating new user", rz.Err(err))
		err = NewError(ErrorCreatingPendingUser)
		return
	}

	return
}
