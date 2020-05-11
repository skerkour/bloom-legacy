package users

import (
	"context"
	"encoding/base64"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
	"gitlab.com/bloom42/lily/crypto"
)

func SaveMasterKey(ctx context.Context, tx *sqlx.Tx, masterKey []byte) error {
	return saveKey(ctx, tx, MASTER_KEY_KEY, masterKey)
}

func SavePrivateKey(ctx context.Context, tx *sqlx.Tx, privateKey crypto.PrivateKey) error {
	return saveKey(ctx, tx, PRIVATE_KEY_KEY, privateKey)
}

func SavePublicKey(ctx context.Context, tx *sqlx.Tx, publicKey crypto.PublicKey) error {
	return saveKey(ctx, tx, PUBLIC_KEY_KEY, publicKey)
}

func saveKey(ctx context.Context, tx *sqlx.Tx, preferenceKey string, key []byte) error {
	encoded := base64.StdEncoding.EncodeToString(key)
	return preferences.Set(ctx, tx, preferenceKey, encoded)
}
