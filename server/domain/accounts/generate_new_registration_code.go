package accounts

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
	"time"
)

func GenerateNewRegistrationCode(ctx context.Context, tx *sqlx.Tx, pendingAccount *PendingAccount) (string, twirp.Error) {
	logger := rz.FromCtx(ctx)
	var err error

	now := time.Now().UTC()
	verificationCode, err := rand.StringAlph(alphabetDigits, 8)
	if err != nil {
		logger.Error("accounts.GenerateNewRegistrationCode: error generating verification code", rz.Err(err))
		return "", twirp.InternalError(ErrorSendingNewRegistrationCode)
	}

	// TODO: update params
	codeHash, err := argon2id.HashPassword([]byte(verificationCode), argon2id.DefaultHashPasswordParams)
	if err != nil {
		logger.Error("accounts.GenerateNewRegistrationCode: hashing verification code", rz.Err(err))
		return "", twirp.InternalError(ErrorSendingNewRegistrationCode)
	}

	pendingAccount.VerificationCodeHash = codeHash
	pendingAccount.UpdatedAt = now

	queryUpdatePendingAccount := "UPDATE pending_accounts SET verification_code_hash = $1, updated_at = $2 WHERE id = $3"
	_, err = tx.Exec(queryUpdatePendingAccount, pendingAccount.VerificationCodeHash, pendingAccount.UpdatedAt, pendingAccount.ID)
	if err != nil {
		logger.Error("accounts.GenerateNewRegistrationCode: updateing pending account", rz.Err(err), rz.String("pending_account_id", pendingAccount.ID))
		return "", twirp.InternalError(ErrorSendingNewRegistrationCode)
	}
	return verificationCode, nil
}
