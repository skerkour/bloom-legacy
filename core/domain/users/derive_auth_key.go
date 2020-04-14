package users

import (
	"gitlab.com/bloom42/lily/crypto"
)

// TODO: good values (key derivation with blake2b)
// return nil if encounter an error
func deriveAuthKey(username, password []byte) []byte {
	authKeySalt := padOrTrimBytes(username, 64)
	authKeySalt = append(authKeySalt, []byte("@bloom42.com")...)

	key, err := crypto.DeriveKeyFromPassword(password, authKeySalt, crypto.KeySize512)
	if err != nil {
		return nil
	}

	context := []byte("com.bloom42.bloom/auth_key")
	authKey, err := crypto.DeriveKeyFromKey(key, context, crypto.KeySize256)
	if err != nil {
		return nil
	}

	return authKey
}

// padOrTrimBytes returns (size) bytes from input (data)
// Short data gets zeros prefixed, Long data gets right bits trimmed
func padOrTrimBytes(data []byte, size int) []byte {
	dataLen := len(data)
	if dataLen == size {
		return data
	} else if dataLen > size {
		return data[:size]
	}

	tmp := make([]byte, size)
	copy(tmp[:dataLen], data)
	return tmp
}
