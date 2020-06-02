package users

import (
	"context"
	"encoding/base64"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func startSession(ctx context.Context, tx *sqlx.Tx, userID uuid.UUID, device SessionDevice) (ret *Session, token string, err error) {
	logger := rz.FromCtx(ctx)

	newSessionID, secret, hash, salt, err := newSession()
	defer crypto.Zeroize(secret) // wipe secret from memory
	if err != nil {
		logger.Error("users.StartSession: generating new session", rz.Err(err))
		err = NewError(ErrorSingingIn)
		return
	}

	now := time.Now().UTC()
	ret = &Session{
		ID:         newSessionID,
		CreatedAt:  now,
		UpdatedAt:  now,
		Hash:       hash,
		Salt:       salt,
		UserID:     userID,
		DeviceOS:   device.OS,
		DeviceType: device.Type,
	}

	queryCreateSession := `INSERT INTO sessions
	(id, created_at, updated_at, user_id, hash, salt, device_os, device_type)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`
	_, err = tx.Exec(queryCreateSession, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.UserID,
		ret.Hash, ret.Salt, ret.DeviceOS, ret.DeviceType)
	if err != nil {
		logger.Error("users.StartSession: inserting new session", rz.Err(err))
		err = NewError(ErrorSingingIn)
		return
	}

	tokenByte := append(ret.ID.Bytes(), secret...)
	token = base64.StdEncoding.EncodeToString(tokenByte)

	return
}

func newSession() (id uuid.UUID, secret, hash, salt []byte, err error) {
	id, err = uuid.NewRandom()
	if err != nil {
		return
	}

	if secret, err = crypto.RandBytes(crypto.KeySize512); err != nil {
		return
	}
	if salt, err = crypto.RandBytes(crypto.KeySize512); err != nil {
		return
	}

	hash, err = hashSession(id, salt, secret)
	return
}
