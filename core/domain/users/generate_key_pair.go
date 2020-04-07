package users

import (
	"io"
)

func generateKeyPair(rand io.Reader) (publicKey, privateKey *[32]byte, err error) {
	publicKey = new([32]byte)
	privateKey = new([32]byte)
	_, err = io.ReadFull(rand, privateKey[:])
	if err != nil {
		publicKey = nil
		privateKey = nil
		return
	}
	return publicKey, privateKey, err
}
