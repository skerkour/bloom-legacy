package objects

import (
	"encoding/base64"
	"encoding/json"

	"github.com/golang/snappy"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/lily/crypto"
)

// compress and encrypt encodes to JSON, compress and encrypt with an unique key
// an object
func compressAndEncryptObject(object StoredObject, masterKey []byte, compressionAlgo compressionAlgorithm) (*model.ObjectInput, error) {
	// TODO: improve AD
	var ret *model.ObjectInput

	objectData, err := json.Marshal(object)
	if err != nil {
		return ret, err
	}

	objectID, err := base64.StdEncoding.DecodeString(object.ID)
	if err != nil {
		return ret, err
	}

	// compress object
	compressedObjectData := snappy.Encode(nil, objectData)
	crypto.Zeroize(objectData)
	algorithm := "snappy+xchacha20-poly1305"

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
	encryptedObject := objectCipher.Seal(nil, objectNonce, compressedObjectData, objectID)
	crypto.Zeroize(compressedObjectData)

	// encrypt objectKey
	objectKeyCipher, err := crypto.NewAEAD(masterKey)
	if err != nil {
		return ret, err
	}
	objectKeyNonce, err := crypto.DeriveKeyFromKey(objectNonce, objectID, crypto.AEADNonceSize)
	if err != nil {
		return ret, err
	}
	encryptedKey := objectKeyCipher.Seal(nil, objectKeyNonce, objectKey, objectID)

	// wipe objectKey from memory
	crypto.Zeroize(objectKey)

	ret = &model.ObjectInput{
		Algorithm:     algorithm,
		EncryptedData: encryptedObject,
		EncryptedKey:  encryptedKey,
		Nonce:         objectNonce,
	}
	return ret, nil
}
