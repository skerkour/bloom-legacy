package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
	"time"
)

func GenerateNewRegistrationCode(ctx context.Context, tx *sqlx.Tx, pendingUser *PendingUser) (string, twirp.Error) {
	logger := rz.FromCtx(ctx)
	var err error

	now := time.Now().UTC()
	verificationCode, err := rand.StringAlph(alphabetDigits, 8)
	if err != nil {
		logger.Error("users.GenerateNewRegistrationCode: error generating verification code", rz.Err(err))
		return "", twirp.InternalError(ErrorSendingNewRegistrationCode)
	}

	// TODO: update params
	codeHash, err := argon2id.HashPassword([]byte(verificationCode), argon2id.DefaultHashPasswordParams)
	if err != nil {
		logger.Error("users.GenerateNewRegistrationCode: hashing verification code", rz.Err(err))
		return "", twirp.InternalError(ErrorSendingNewRegistrationCode)
	}

	pendingUser.VerificationCodeHash = codeHash
	pendingUser.UpdatedAt = now

	queryUpdatePendingUser := "UPDATE pending_users SET verification_code_hash = $1, updated_at = $2 WHERE id = $3"
	_, err = tx.Exec(queryUpdatePendingUser, pendingUser.VerificationCodeHash, pendingUser.UpdatedAt, pendingUser.ID)
	if err != nil {
		logger.Error("users.GenerateNewRegistrationCode: updateing pending user", rz.Err(err), rz.String("pending_user_id", pendingUser.ID))
		return "", twirp.InternalError(ErrorSendingNewRegistrationCode)
	}
	return verificationCode, nil
}