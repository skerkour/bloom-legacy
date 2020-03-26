package users

import (
	"context"
	"encoding/base64"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func StartSession(ctx context.Context, tx *sqlx.Tx, userID string, device SessionDevice) (Session, string, error) {
	logger := rz.FromCtx(ctx)
	ret := Session{}
	var token string

	tokenSecret, err := rand.Bytes(uint64(consts.SESSION_TOKEN_BYTES))
	if err != nil {
		logger.Error("users.StartSession: generating sessions token", rz.Err(err))
		return ret, token, NewError(ErrorSingingIn)
	}

	// TODO: update params
	tokenHash, err := argon2id.HashPassword(tokenSecret, argon2id.DefaultHashPasswordParams)
	if err != nil {
		logger.Error("users.StartSession: hashing auth key", rz.Err(err))
		return ret, token, NewError(ErrorSingingIn)
	}

	now := time.Now().UTC()
	newUuid := uuid.New()

	ret = Session{
		ID:         newUuid.String(),
		CreatedAt:  now,
		UpdatedAt:  now,
		TokenHash:  tokenHash,
		UserID:     userID,
		DeviceOS:   device.OS,
		DeviceType: device.Type,
	}

	queryCreateSession := `INSERT INTO sessions
	(id, created_at, updated_at, user_id, token_hash, device_os, device_type)
	VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err = tx.Exec(queryCreateSession, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.UserID,
		ret.TokenHash, ret.DeviceOS, ret.DeviceType)
	if err != nil {
		logger.Error("users.StartSession: inserting new session", rz.Err(err))
		return ret, token, NewError(ErrorSingingIn)
	}

	token = base64.StdEncoding.EncodeToString([]byte(fmt.Sprintf("%s:%s", ret.ID, tokenSecret)))
	return ret, token, nil
}
