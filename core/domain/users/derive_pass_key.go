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

	// username is used as a salt
	info := append([]byte("com.bloom42.bloom/pass_key"), username...)
	authKey, err := crypto.DeriveKeyFromKey(key, info, crypto.KeySize256)
	if err != nil {
		return nil
	}

	return authKey
}
