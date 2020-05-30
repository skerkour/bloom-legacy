package keys

import (
	"gitlab.com/bloom42/gobox/crypto"
)

func DerivePasswordKeyFromPassword(password, username []byte) (key []byte, err error) {
	salt := username

	key, err = crypto.DeriveKeyFromPassword(password, salt, PASSWORD_KDF_PARAMS)
	return
}

func DeriveWrapKeyFromPasswordKey(passwordKey, username []byte) (key []byte, err error) {
	message := append([]byte("com.bloom42.bloom/wrap_key"), username...)
	key, err = crypto.DeriveKeyFromKey(key, message, crypto.KeySize256)
	return
}

func DeriveAuthKeyFromPasswordKey(passwordKey, username []byte) (key []byte, err error) {
	message := append([]byte("com.bloom42.bloom/auth_key"), username...)
	key, err = crypto.DeriveKeyFromKey(key, message, crypto.KeySize512)
	return
}
