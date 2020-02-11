package users

import (
	"context"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/bloom/server/config"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func CreatePendingUser(ctx context.Context, tx *sqlx.Tx, displayName, email string) (PendingUser, string, error) {
	logger := rz.FromCtx(ctx)
	var existingUser int
	var err error

	// clean and validate params
	if err = validator.UserDisplayName(displayName); err != nil {
		return PendingUser{}, "", NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	email = strings.ToLower(email)
	email = strings.TrimSpace(email)
	if err = validator.UserEmail(email, config.DisposableEmailDomains); err != nil {
		return PendingUser{}, "", NewErrorMessage(ErrorInvalidArgument, err.Error()) // twirp.InvalidArgumentError("email", err.Error())
	}

	// check if email does not already exist
	queryCountExistingEmails := "SELECT COUNT(*) FROM users WHERE email = $1"
	err = tx.Get(&existingUser, queryCountExistingEmails, email)
	if err != nil {
		logger.Error("users.CreatePendingUser: error fetching existing emails counts", rz.Err(err))
		return PendingUser{}, "", NewError(ErrorCreatingPendingUser)
	}

	if existingUser != 0 {
		return PendingUser{}, "", NewError(ErrorEmailAlreadyExists) // twirp.InvalidArgumentError("email", fmt.Sprintf("user with email: '%s' already exists", email))
	}

	now := time.Now().UTC()
	newUuid := uuid.New()
	verificationCode, err := rand.StringAlph(userVerificationCodeAlphabet, 8)
	if err != nil {
		logger.Error("users.CreatePendingUser: error generating verification code", rz.Err(err))
		return PendingUser{}, "", NewError(ErrorCreatingPendingUser)
	}

	// TODO: update params
	codeHash, err := argon2id.HashPassword([]byte(verificationCode), argon2id.DefaultHashPasswordParams)
	if err != nil {
		logger.Error("users.CreatePendingUser: hashing verification code", rz.Err(err))
		return PendingUser{}, "", NewError(ErrorCreatingPendingUser)
	}
	ret := PendingUser{
		ID:                   newUuid.String(),
		CreatedAt:            now,
		UpdatedAt:            now,
		Email:                email,
		DisplayName:          displayName,
		VerificationCodeHash: codeHash,
		FailedVerifications:  0,
		Verified:             false,
	}

	queryCreatePendingUser := `INSERT INTO pending_users
		(id, created_at, updated_at, email, display_name, verification_code_hash, failed_verifications, verified)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`
	_, err = tx.Exec(queryCreatePendingUser, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Email,
		ret.DisplayName, ret.VerificationCodeHash, ret.FailedVerifications, ret.Verified)
	if err != nil {
		logger.Error("error creating new user", rz.Err(err))
		return ret, "", NewError(ErrorCreatingPendingUser)
	}
	return ret, verificationCode, nil
}
