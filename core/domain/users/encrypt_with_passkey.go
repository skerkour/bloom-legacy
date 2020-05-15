package users

import (
	"gitlab.com/bloom42/lily/crypto"
)

func encryptWithWrapKey(wrapKey, plaintext []byte) (ciphertext, nonce []byte, err error) {
	nonce, err = crypto.NewAEADNonce()
	if err != nil {
		return
	}
	cipher, err := crypto.NewAEAD(wrapKey)
	if err != nil {
		return
	}
	ciphertext = cipher.Seal(nil, nonce, plaintext, nil)
	return
}
