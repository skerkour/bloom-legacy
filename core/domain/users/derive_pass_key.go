package users

import (
	"golang.org/x/crypto/argon2"
	"golang.org/x/crypto/blake2b"
)

// TODO
func derivePassKey(username, password []byte) []byte {
	clientSalt := padOrTrimBytes(username, 64)

	key := argon2.Key(password, clientSalt, 3, 32*1024, 4, 32)
	keyID := []byte("blm_auth")
	key = append(key, keyID...)

	blake2bHash, err := blake2b.New512(nil)
	if err != nil {
		return nil
	}

	return blake2bHash.Sum(key)
}
