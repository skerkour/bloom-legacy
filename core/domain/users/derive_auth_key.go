package users

import (
	"gitlab.com/bloom42/lily/crypto/kdf"
)

// TODO: good values (key derivation with blake2b)
// return nil if encounter an error
func deriveAuthKey(username, password []byte) []byte {
	authKeySalt := padOrTrimBytes(username, 64)

	key, err := kdf.DeriveFromPassword(password, authKeySalt, kdf.KeySize256)
	if err != nil {
		return nil
	}

	context := []byte("com.bloom42.bloom/auth_key")
	authKey, err := kdf.DeriveFromKey(key, context, kdf.KeySize256)
	if err != nil {
		return nil
	}

	return authKey
}

// padOrTrimBytes returns (size) bytes from input (data)
// Short data gets zeros prefixed, Long data gets right bits trimmed
func padOrTrimBytes(data []byte, size int) []byte {
	DataLen := len(data)
	if DataLen == size {
		return data
	}
	if DataLen > size {
		return data[:size]
	}

	tmp := make([]byte, size)
	copy(tmp[:DataLen], data)
	return tmp
}
