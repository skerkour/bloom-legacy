package keys

import (
	"context"
	"encoding/base64"
	"errors"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
	"gitlab.com/bloom42/gobox/crypto"
)

func FindUserMasterKey(ctx context.Context, tx *sqlx.Tx) ([]byte, error) {
	return findUserKey(ctx, tx, PREFERENCES_KEY_MASTER_KEY)
}

func FindUserPrivateKey(ctx context.Context, tx *sqlx.Tx) ([]byte, error) {
	return findUserKey(ctx, tx, PREFERENCES_KEY_PRIVATE_KEY)
}

func FindUserPublicKey(ctx context.Context, tx *sqlx.Tx) ([]byte, error) {
	return findUserKey(ctx, tx, PREFERENCES_KEY_PUBLIC_KEY)
}

func findUserKey(ctx context.Context, tx *sqlx.Tx, preferencesKey string) ([]byte, error) {
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

// SaveUserMasterKey saves the masterKey in preferences
func SaveUserMasterKey(ctx context.Context, tx *sqlx.Tx, masterKey []byte) error {
	return saveUserKey(ctx, tx, PREFERENCES_KEY_MASTER_KEY, masterKey)
}

// SaveUserPrivateKey saves the privateKey in preferences
func SaveUserPrivateKey(ctx context.Context, tx *sqlx.Tx, privateKey crypto.PrivateKey) error {
	return saveUserKey(ctx, tx, PREFERENCES_KEY_PRIVATE_KEY, privateKey)
}

// SaveUserPublicKey saves the public key in preferences
func SaveUserPublicKey(ctx context.Context, tx *sqlx.Tx, publicKey crypto.PublicKey) error {
	return saveUserKey(ctx, tx, PREFERENCES_KEY_PUBLIC_KEY, publicKey)
}

func saveUserKey(ctx context.Context, tx *sqlx.Tx, preferenceKey string, key []byte) error {
	encoded := base64.StdEncoding.EncodeToString(key)
	return preferences.Set(ctx, tx, preferenceKey, encoded)
}
