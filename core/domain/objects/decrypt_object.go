package objects

import (
	"encoding/json"
	"errors"

	"github.com/golang/snappy"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/lily/crypto"
)

func decryptObject(encryptedObject *model.ObjectInput, masterKey []byte) (ret *StoredObject, err error) {
	// TODO: improve AD
	// TODO: verify data
	if encryptedObject.Algorithm != DEFAULT_ALGORITHM_STRING {
		err = errors.New("Invalid algorithm")
		return
	}

	// decrypt objectKey
	objectKeyCipher, err := crypto.NewAEAD(masterKey)
	if err != nil {
		return ret, err
	}
	objectKeyNonce, err := crypto.DeriveKeyFromKey(encryptedObject.Nonce, encryptedObject.ID, crypto.AEADNonceSize)
	if err != nil {
		return
	}
	objectKey, err := objectKeyCipher.Open(nil, objectKeyNonce, encryptedObject.EncryptedKey, encryptedObject.ID)
	if err != nil {
		err = errors.New("Error decrypting object key")
		return
	}

	// decrypt object data
	objectDataCipher, err := crypto.NewAEAD(objectKey)
	if err != nil {
		return ret, err
	}
	compressedObjectData, err := objectDataCipher.Open(nil, encryptedObject.Nonce, encryptedObject.EncryptedData, encryptedObject.ID)
	if err != nil {
		err = errors.New("Error decrypting object")
		return
	}
	crypto.Zeroize(objectKey)

	// decompress object
	objectData, err := snappy.Decode(nil, compressedObjectData)
	if err != nil {
		err = errors.New("Error decompressing object")
		return
	}
	crypto.Zeroize(compressedObjectData)

	ret = &StoredObject{}
	err = json.Unmarshal(objectData, ret)
	if err != nil {
		err = errors.New("Error parsing object")
		return
	}

	return ret, nil
}
