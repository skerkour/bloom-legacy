package users

import (
	"context"
	"encoding/base64"
	"errors"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
)

func FindMasterKey(ctx context.Context, tx *sqlx.Tx) ([]byte, error) {
	return findKey(ctx, tx, MASTER_KEY_KEY)
}

func FindPrivateKey(ctx context.Context, tx *sqlx.Tx) ([]byte, error) {
	return findKey(ctx, tx, PRIVATE_KEY_KEY)
}

func FindPublicKey(ctx context.Context, tx *sqlx.Tx) ([]byte, error) {
	return findKey(ctx, tx, PUBLIC_KEY_KEY)
}

func findKey(ctx context.Context, tx *sqlx.Tx, preferencesKey string) ([]byte, error) {
	encodedKey, err := preferences.Get(context.Background(), tx, preferencesKey)
	if err != nil {
		return nil, err
	}
	if encodedKey == nil {
		return nil, errors.New("Key not found")
	}

	decodedKey, err := base64.StdEncoding.DecodeString(*encodedKey)
	return decodedKey, err
}
