package users

import (
	"gitlab.com/bloom42/lily/crypto/kdf"
)

// TODO
func derivePassKey(username, password []byte) []byte {
	authKeySalt := padOrTrimBytes(username, 64)

	key, err := kdf.DeriveFromPassword(password, authKeySalt, kdf.KeySize512)
	if err != nil {
		return nil
	}

	context := []byte("com.bloom42.bloom/pass_key")
	authKey, err := kdf.DeriveFromKey(key, context, kdf.KeySize256)
	if err != nil {
		return nil
	}

	return authKey
}
