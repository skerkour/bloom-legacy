package accounts

import (
	"github.com/jmoiron/sqlx"
	"time"
)

func CreatePendingAccount(tx *sqlx.Tx, displayName, email string) (PendingAccount, error) {
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
		VALUES (?, ?, ?, ?, ?, ?, ?, ?)`
	tx.MustExec(query, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Email, ret.DisplayName, ret.VerificationCodeHash, ret.Trials, ret.Verified)
	return ret, nil
}
