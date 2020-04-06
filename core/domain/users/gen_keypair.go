package users

func genKeyPair() ([]byte, []byte, error) {
	publicKey := make([]byte, 12)
	privateKey := make([]byte, 12)
	return publicKey, privateKey, nil
}
