package objects

import (
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/lily/crypto"
)

// compress and encrypt encodes to JSON, compress and encrypt with an unique key
// an object
func compressAndEncrypt(object StoredObject, masterKey []byte, compressionAlgo compressionAlgorithm) (*model.ObjectInput, error) {
	// TODO:
	// * objectID
	// * compression
	// * algorithm
	var ret *model.ObjectInput
	objectData, err := json.Marshal(object)
	if err != nil {
		return ret, err
	}
	var objectID []byte // todo

	// encrypt object
	objectNonce, err := crypto.NewAEADNonce()
	objectKey, err := crypto.NewAEADKey()
	objectCipher, err := crypto.NewAEAD(objectKey)
	encryptedObject := objectCipher.Seal(nil, objectNonce, objectData, nil)

	// encrypt objectKey
	objectKeyCipher, err := crypto.NewAEAD(masterKey)
	objectKeyNonce, err := crypto.DeriveKeyFromKey(objectNonce, objectID, crypto.AEADNonceSize)
	encryptedKey := objectKeyCipher.Seal(nil, objectKeyNonce, objectKey, nil)

	crypto.Zeroize(objectKey)

	ret = &model.ObjectInput{
		EncryptedData: encryptedObject,
		EncryptedKey:  encryptedKey,
		Nonce:         objectNonce,
	}
	return ret, nil
}
