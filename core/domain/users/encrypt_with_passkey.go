package users

// TODO
func encryptWithPassKey(passkey, plaintext []byte) (ciphertext, nonce []byte, err error) {
	ciphertext = make([]byte, len(plaintext))
	nonce = make([]byte, 24)
	return ciphertext, nonce, err
}
