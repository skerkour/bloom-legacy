package accounts

import (
	"context"
	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
	"time"
)

func StartSession(ctx context.Context, tx *sqlx.Tx, accountID, ipAddr, userAgent string) (Session, string, twirp.Error) {
	logger := rz.FromCtx(ctx)
	ret := Session{}

	token, err := rand.TokenBase64(uint64(consts.SESSION_TOKEN_BYTES))
	if err != nil {
		logger.Error("accounts.StartSession: generating sessions token", rz.Err(err))
		return ret, token, twirp.NewError(twirp.Internal, ErrorSingingInMsg)
	}

	// TODO: update params
	tokenHash, err := argon2id.HashPassword([]byte(token), argon2id.DefaultHashPasswordParams)
	if err != nil {
		logger.Error("accounts.StartSession: hashing auth key", rz.Err(err))
		return ret, token, twirp.InternalError(ErrorSingingInMsg)
	}

	now := time.Now().UTC()
	newUuid := uuid.New()

	ret = Session{
		ID:        newUuid.String(),
		CreatedAt: now,
		UpdatedAt: now,
		TokenHash: tokenHash,
		AccountID: accountID,
		IPAddr:    ipAddr,
		UserAgent: userAgent,
	}

	queryCreateSession := `INSERT INTO sessions
	(id, created_at, updated_at, account_id, token_hash, ip, user_agent)
	VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err = tx.Exec(queryCreateSession, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.AccountID, ret.TokenHash, ret.IPAddr, ret.UserAgent)
	if err != nil {
		logger.Error("accounts.StartSession: inserting new session", rz.Err(err))
		return ret, token, twirp.InternalError(ErrorSingingInMsg)
	}

	return ret, token, nil
}
