package users

import (
	"golang.org/x/crypto/argon2"
	"golang.org/x/crypto/blake2b"
)

// TODO: good values (key derivation with blake2b)
// return nil if encounter an error
func deriveAuthKey(username, password []byte) []byte {
	authKeySalt := padOrTrimBytes(username, 64)

	key := argon2.Key(password, authKeySalt, 3, 32*1024, 4, 32)
	keyID := []byte("blm_auth")
	key = append(key, keyID...)

	blake2bHash, err := blake2b.New256(nil)
	if err != nil {
		return nil
	}

	return blake2bHash.Sum(key)
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
