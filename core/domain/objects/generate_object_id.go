package objects

import (
	"gitlab.com/bloom42/lily/crypto"
)

func GenerateObjectID(username []byte) (ret []byte, err error) {
	randData, err := crypto.RandBytes(crypto.KeySize512)
	if err != nil {
		return
	}
	ret, err = crypto.DeriveKeyFromKey(username, randData, crypto.KeySize512)
	return
}
