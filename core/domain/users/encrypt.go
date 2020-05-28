package users

import (
	"gitlab.com/bloom42/gobox/crypto"
)

func Encrypt(key, plaintext []byte) (ciphertext, nonce []byte, err error) {
	nonce, err = crypto.NewAEADNonce()
	if err != nil {
		return
	}
	cipher, err := crypto.NewAEAD(key)
	if err != nil {
		return
	}
	ciphertext = cipher.Seal(nil, nonce, plaintext, nil)
	return
}

func Decrypt(key, nonce, ciphertext []byte) (plaintext []byte, err error) {
	cipher, err := crypto.NewAEAD(key)
	if err != nil {
		return
	}
	plaintext, err = cipher.Open(nil, nonce, ciphertext, nil)
	return
}
