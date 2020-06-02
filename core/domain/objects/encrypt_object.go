package objects

import (
	"encoding/json"

	"github.com/golang/snappy"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/gobox/crypto"
)

// compress and encrypt encodes to JSON, compress and encrypt with an unique key
// an object
func encryptObject(object Object, masterKey []byte, compressionAlgo compressionAlgorithm) (*model.ObjectInput, error) {
	// TODO: improve AD
	// TODO: verify data
	var ret *model.ObjectInput

	objectData, err := json.Marshal(object)
	if err != nil {
		return ret, err
	}

	// compress object
	compressedObjectData := snappy.Encode(nil, objectData)
	crypto.Zeroize(objectData)

	// encrypt object
	objectNonce, err := crypto.NewAEADNonce()
	if err != nil {
		return ret, err
	}
	objectKey, err := crypto.NewAEADKey()
	if err != nil {
		return ret, err
	}
	objectCipher, err := crypto.NewAEAD(objectKey)
	if err != nil {
		return ret, err
	}
	encryptedObject := objectCipher.Seal(nil, objectNonce, compressedObjectData, object.ID)
	crypto.Zeroize(compressedObjectData)

	// encrypt objectKey
	objectKeyCipher, err := crypto.NewAEAD(masterKey)
	if err != nil {
		return ret, err
	}
	objectKeyNonce, err := crypto.DeriveKeyFromKey(objectNonce, object.ID, crypto.AEADNonceSize)
	if err != nil {
		return ret, err
	}
	encryptedKey := objectKeyCipher.Seal(nil, objectKeyNonce, objectKey, object.ID)

	// wipe objectKey from memory
	crypto.Zeroize(objectKey)

	ret = &model.ObjectInput{
		ID:            object.ID,
		Algorithm:     DEFAULT_ALGORITHM_STRING,
		EncryptedData: encryptedObject,
		EncryptedKey:  encryptedKey,
		Nonce:         objectNonce,
	}
	return ret, nil
}
