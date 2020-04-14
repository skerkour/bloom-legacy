package users

import (
	"gitlab.com/bloom42/lily/crypto"
)

// TODO
func derivePassKey(username, password []byte) []byte {
	authKeySalt := padOrTrimBytes(username, 64)
	authKeySalt = append(authKeySalt, []byte("@bloom42.com")...)

	key, err := crypto.DeriveKeyFromPassword(password, authKeySalt, crypto.KeySize512)
	if err != nil {
		return nil
	}

	context := []byte("com.bloom42.bloom/pass_key")
	authKey, err := crypto.DeriveKeyFromKey(key, context, crypto.KeySize256)
	if err != nil {
		return nil
	}

	return authKey
}
